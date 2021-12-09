use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let map: Map = input.parse().unwrap();
       let mut sum = 0;
       for x in 0..100 {
           for y in 0..100 {
               let p = Point(x, y);
               let h = map.get_height(&p);

               if p.adj().iter().all(|a| map.get_height(a) > h) {
                   sum += h + 1;
               }
           }
       }
       format!("{}", sum)
   }

   fn solve_2(&self, input: String) -> String {
       let map: Map = input.parse().unwrap();

       let mut low_points = Vec::new();
       for x in 0..100 {
           for y in 0..100 {
               let p = Point(x, y);
               let h = map.get_height(&p);

               if p.adj().iter().all(|a| map.get_height(a) > h) {
                   low_points.push(Point(x, y));
               }
           }
       }

       log::debug!("{:?}", low_points);

       let mut basins = Vec::new();
       let mut visited: HashSet<Point> = HashSet::new();
       while let Some(p) = low_points.pop() {
           if visited.contains(&p) {
               continue
           }

           let mut basin = HashSet::new();
           let mut stack = vec![p];
           while let Some(c) = stack.pop() {
               for a in c.adj() {
                   if map.get_height(&a) < 9 && !basin.contains(&a) {
                       basin.insert(a);
                       visited.insert(a);
                       stack.push(a);
                   }
               }
           }

           basins.push(basin.len());
       }

       basins.sort_unstable();
       basins.reverse();

       log::debug!("{:?}", basins);

       let res = basins.into_iter()
           .take(3)
           .reduce(|a, b| a * b)
           .unwrap();

       format!("{}", res)
   }
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
    fn get_height(&self, p: &Point) -> u32 {
        self.grid.get(p).copied().unwrap_or(9)
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
