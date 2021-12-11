use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let mut map: Map = input.parse().unwrap();

       let total: u32 = (0..100).map(|_| map.step()).sum();
       format!("{}", total)
   }

   fn solve_2(&self, input: String) -> String {
       let mut map: Map = input.parse().unwrap();

       for i in 1.. {
           map.step();

           if map.all_flash() {
               return format!("{}", i);
           }
       }

       unreachable!()
   }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

impl Point {
    fn adj(&self) -> [Point; 8] {
        let (x, y) = (self.0, self.1);
        let n = Point(x, y-1);
        let e = Point(x+1, y);
        let s = Point(x, y+1);
        let w = Point(x-1, y);
        let ne = Point(x+1, y-1);
        let se = Point(x+1, y+1);
        let sw = Point(x-1, y+1);
        let nw = Point(x-1, y-1);

        [n, ne, e, se, s, sw, w, nw]
    }
}

struct Map {
    grid: HashMap<Point, u32>,
}

impl Map {
    fn step(&mut self) -> u32 {
        log::debug!("{}", self);

        for x in 0..10 {
            for y in 0..10 {
                *self.grid.entry(Point(x, y)).or_default() += 1;
            }
        }

        let mut flashed: HashSet<Point> = HashSet::new();
        loop {
            let mut flashed_now: HashSet<Point> = HashSet::new();
            for x in 0..10 {
                for y in 0..10 {
                    let p = Point(x, y);
                    if self.grid[&p] > 9 && !flashed.contains(&p) {
                        flashed.insert(p);
                        flashed_now.insert(p);
                    }
                }
            }

            for p in &flashed_now {
                for a in p.adj() {
                    *self.grid.entry(a).or_default() += 1;
                }
            }

            if flashed_now.is_empty() {
                break
            }
        }

        for p in &flashed {
            *self.grid.entry(*p).or_default() = 0;
        }

        flashed.len() as u32
    }

    fn all_flash(&self) -> bool {
        for x in 0..10 {
            for y in 0..10 {
                let l = self.grid[&Point(x, y)];
                if l > 0 {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        for (y, line) in (0..).zip(s.lines()) {
            for (x, c) in (0..).zip(line.chars()) {
                grid.insert(Point(x, y), c.to_digit(10).ok_or(())?);
            }
        }
        Ok(Map { grid })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                write!(f, "{}", self.grid.get(&Point(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
