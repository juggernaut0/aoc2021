use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let mut map: Map = input.parse().unwrap();

       for i in 1.. {
           log::debug!("\n{}", map);
           if map.step() {
               return format!("{}", i)
           }
       }

       unreachable!()
   }

   fn solve_2(&self, _input: String) -> String {
       "Merry Christmas!".to_string()
   }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Right,
    Down,
}

struct Map {
    cukes: HashMap<Point, Tile>,
    bounds_x: i32,
    bounds_y: i32,
}

impl Map {
    fn is_occupied(&self, p: Point) -> bool {
        self.cukes.contains_key(&p)
    }

    fn step(&mut self) -> bool {
        let right_stopped = !self.step_right();
        let down_stopped = !self.step_down();
        right_stopped && down_stopped
    }

    fn step_right(&mut self) -> bool {
        let mut new = HashMap::new();
        let mut moved = false;
        for (p, t) in &self.cukes {
            if t == &Tile::Right {
                log::trace!("right");
                let d = Point((p.0 + 1) % self.bounds_x, p.1);
                if !self.is_occupied(d) {
                    new.insert(d, Tile::Right);
                    moved = true;
                } else {
                    new.insert(*p, Tile::Right);
                }
            } else {
                new.insert(*p, Tile::Down);
            }
        }
        self.cukes = new;
        moved
    }

    fn step_down(&mut self) -> bool {
        let mut new = HashMap::new();
        let mut moved = false;
        for (p, t) in &self.cukes {
            if t == &Tile::Down {
                log::trace!("down");
                let d = Point(p.0, (p.1 + 1) % self.bounds_y);
                if !self.is_occupied(d) {
                    new.insert(d, Tile::Down);
                    moved = true;
                } else {
                    new.insert(*p, Tile::Down);
                }
            } else {
                new.insert(*p, Tile::Right);
            }
        }
        self.cukes = new;
        moved
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cukes = HashMap::new();
        for (line, y) in s.lines().zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                let p = Point(x, y);
                match c {
                    '>' => { cukes.insert(p, Tile::Right); }
                    'v' => { cukes.insert(p, Tile::Down); }
                    _ => {}
                };
            }
        }
        let bounds_x = s.lines().next().unwrap().len() as i32;
        let bounds_y = s.lines().count() as i32;
        Ok(Map { cukes, bounds_x, bounds_y })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.bounds_y {
            for x in 0..self.bounds_x {
                let p = Point(x, y);
                if let Some(t) = self.cukes.get(&p) {
                    match t {
                        Tile::Right => write!(f, ">")?,
                        Tile::Down => write!(f, "v")?,
                    };
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn ex() {
        crate::init_test_logging();
        let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
".to_string();

        assert_eq!("58", Solution.solve_1(input));
    }
}