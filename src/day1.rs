#![allow(unused_variables)]

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let depths: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
       depths.windows(2).filter(|it| it[0] < it[1]).count().to_string()
   }

   fn solve_2(&self, input: String) -> String {
       let depths: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
       let windows: Vec<i32> = depths.windows(3).map(|it| it.iter().sum()).collect();
       windows.windows(2).filter(|it| it[0] < it[1]).count().to_string()
   }
}
