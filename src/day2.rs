#![allow(unused_variables)]

use std::str::FromStr;
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let (x, y) = parse_lines(&input)
           .fold((0, 0), |(x, y), b| {
               match b {
                   Direction::Forward(n) => (x + n, y),
                   Direction::Down(n) => (x, y + n),
               }
           });
       (x * y).to_string()
   }

   fn solve_2(&self, input: String) -> String {
       let (x, y, _) = parse_lines(&input)
           .fold((0, 0, 0), |(pos, depth, aim), b| {
               match b {
                   Direction::Forward(n) => (pos + n, depth + aim*n, aim),
                   Direction::Down(n) => (pos, depth, aim + n),
               }
           });
       (x * y).to_string()
   }
}

enum Direction {
    Forward(i32),
    Down(i32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let dir = parts.next().ok_or_else(|| s.to_string())?;
        let n: i32 = parts.next().ok_or_else(|| s.to_string())?.parse().map_err(|_| s.to_string())?;
        match dir {
            "forward" => Ok(Direction::Forward(n)),
            "down" => Ok(Direction::Down(n)),
            "up" => Ok(Direction::Down(-n)),
            _ => Err(s.to_string())
        }
    }
}
