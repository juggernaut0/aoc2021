use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let map = input.parse().unwrap();

       solve(map, Point(99, 99))
   }

   fn solve_2(&self, input: String) -> String {
       let map: Map = input.parse().unwrap();

       let mut final_grid = HashMap::new();
       for (Point(x, y), c) in map.grid {
           for rx in 0..5 {
               for ry in 0..5 {
                   let rp = Point(x + 100*rx, y + 100*ry);
                   let mut rc = c + (rx as u32) + (ry as u32);
                   if rc > 9 {
                       rc -= 9;
                   }
                   final_grid.insert(rp, rc);
               }
           }
       }

       solve(Map { grid: final_grid }, Point(499, 499))
   }
}

fn solve(map: Map, goal: Point) -> String {
    let mut q = BinaryHeap::new();
    let p = Path { current: Point(0, 0), cost: 0 };
    q.push(p);

    let mut fastest_to: HashMap<Point, u32> = HashMap::new();

    while let Some(path) = q.pop() {
        let current = path.current;

        if current == goal {
            return format!("{}", path.cost)
        }

        if let Some(c) = fastest_to.get(&current).copied() {
            if c <= path.cost {
                continue
            }
        }
        fastest_to.insert(current, path.cost);

        for a in current.adj() {
            if !fastest_to.contains_key(&a) {
                let new_cost = path.cost + map.get_risk(&a);
                let p = Path { current: a, cost: new_cost };

                q.push(p);
            }
        }
    }

    unreachable!()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

impl Point {
    fn adj(&self) -> [Point; 4] {
        let (x, y) = (self.0, self.1);
        let n = Point(x, y-1);
        let e = Point(x+1, y);
        let s = Point(x, y+1);
        let w = Point(x-1, y);

        [n, e, s, w]
    }
}

struct Map {
    grid: HashMap<Point, u32>,
}

impl Map {
    fn get_risk(&self, p: &Point) -> u32 {
        self.grid.get(p).copied().unwrap_or(1000000)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.insert(Point(x as i32, y as i32), c.to_digit(10).ok_or(())?);
            }
        }
        Ok(Map { grid })
    }
}

struct Path {
    cost: u32,
    current: Point,
}

impl Path {
    fn priority(&self) -> u32 {
        self.cost
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.priority().eq(&other.priority())
    }
}
impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority()).reverse()
    }
}
