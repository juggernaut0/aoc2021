use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let mut counts = HashMap::new();
       for line in parse_lines::<Line>(&input).filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1) {
           for point in line.points() {
               *counts.entry(point).or_insert(0) += 1;
           }
       }

       format!("{}", counts.values().filter(|it| **it >= 2).count())
   }

   fn solve_2(&self, input: String) -> String {
       let mut counts = HashMap::new();
       for line in parse_lines::<Line>(&input) {
           for point in line.points() {
               *counts.entry(point).or_insert(0) += 1;
           }
       }

       format!("{}", counts.values().filter(|it| **it >= 2).count())
   }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point(i32, i32);
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> Vec<Point> {
        let x_dir = match self.start.0.cmp(&self.end.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y_dir = match self.start.1.cmp(&self.end.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let mut points = Vec::new();
        let mut p = self.start;
        loop {
            points.push(p);
            if p == self.end {
                break
            }
            p.0 += x_dir;
            p.1 += y_dir;
        }
        points
    }
}

impl FromStr for Line {
    type Err = String;

    // x,y -> x,y
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }
        let caps = RE.captures(s).ok_or_else(|| format!("Invalid format: {}", s))?;
        let x1 = caps[1].parse().map_err(|_| s.to_string())?;
        let y1 = caps[2].parse().map_err(|_| s.to_string())?;
        let x2 = caps[3].parse().map_err(|_| s.to_string())?;
        let y2 = caps[4].parse().map_err(|_| s.to_string())?;
        Ok(Line { start: Point(x1, y1), end: Point(x2, y2) })
    }
}
