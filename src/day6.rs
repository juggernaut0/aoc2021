use std::collections::HashMap;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       solve(&input, 80)
   }

   fn solve_2(&self, input: String) -> String {
       solve(&input, 256)
   }
}

fn solve(input: &str, steps: u32) -> String {
    let fish_list = input.split(',').map(|it| it.trim().parse().unwrap());
    let mut counts = HashMap::new();
    for fish in fish_list {
        *counts.entry(fish).or_default() += 1;
    }
    let mut school = School { counts };

    for _ in 0..steps {
        school.step();
    }

    format!("{}", school.counts.values().sum::<usize>())
}

struct School {
    counts: HashMap<u8, usize>, // timer value to num fish
}

impl School {
    fn step(&mut self) {
        let mut new = HashMap::new();
        for t in 0..9 {
            let n = self.counts.get(&t).copied().unwrap_or(0);
            if t == 0 {
                *new.entry(8).or_default() += n;
                *new.entry(6).or_default() += n;
            } else {
                *new.entry(t-1).or_default() += n;
            }
        }
        self.counts = new;
    }
}
