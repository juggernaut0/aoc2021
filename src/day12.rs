use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       solve(&input, false)
   }

   fn solve_2(&self, input: String) -> String {
       solve(&input, true)
   }
}

fn solve(input: &str, can_visit_twice: bool) -> String {
    let graph = parse_graph(input, can_visit_twice);

    let mut paths = vec![graph];

    let mut count = 0;
    while let Some(g) = paths.pop() {
        if g.is_finished() {
            log::debug!("{:?}", g.visited);
            count += 1;
        } else {
            paths.extend(g.step());
        }
    }

    format!("{}", count)
}

fn parse_graph(input: &str, can_visit_twice: bool) -> Graph {
    let edges = input.lines()
        .map(|line| {
            let mut parts = line.split('-').map(|it| Room(Rc::new(it.to_string())));
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let pos = Room(Rc::new("start".to_string()));
    let visited = vec![pos.clone()];

    Graph { pos, edges: Rc::new(edges), visited, visited_twice: !can_visit_twice }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Room(Rc<String>);

impl Room {
    fn is_big(&self) -> bool {
        self.0.chars().next().unwrap().is_ascii_uppercase()
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Graph {
    pos: Room,
    edges: Rc<Vec<(Room, Room)>>,
    visited: Vec<Room>,
    visited_twice: bool,
}

impl Graph {
    fn step(&self) -> impl Iterator<Item=Graph> + '_ {
        self.edges.iter()
            .filter_map(move |(a, b)| {
                if a == &self.pos {
                    Some(b.clone())
                } else if b == &self.pos {
                    Some(a.clone())
                } else {
                    None
                }
            })
            .filter_map(move |r| {
                if r.is_big() {
                    Some(self.make_step(r, self.visited_twice))
                } else if self.visited.contains(&r) {
                    if r.0.as_str() == "start" || r.0.as_str() == "end" {
                        None
                    } else if !self.visited_twice {
                        Some(self.make_step(r, true))
                    } else {
                        None
                    }
                } else {
                    Some(self.make_step(r, self.visited_twice))
                }
            })
    }

    fn make_step(&self, pos: Room, visited_twice: bool) -> Graph {
        let mut visited = self.visited.clone();
        visited.push(pos.clone());
        Graph {
            pos,
            edges: Rc::clone(&self.edges),
            visited,
            visited_twice,
        }
    }

    fn is_finished(&self) -> bool {
        self.pos.0.as_str() == "end"
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;
    #[test]
    fn smol_pt2() {
        crate::init_test_logging();
        let inp = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end".to_string();
        assert_eq!("36", &Solution.solve_2(inp));
    }
}