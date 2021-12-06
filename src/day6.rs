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
    let fish_list = input.split(',').map(|it| it.trim().parse::<usize>().unwrap());
    let mut counts = [0; 9];
    for fish in fish_list {
        counts[fish] += 1;
    }
    let mut school = School { counts };

    for _ in 0..steps {
        school.step();
    }

    format!("{}", school.counts.iter().sum::<u64>())
}

struct School {
    counts: [u64; 9], // timer value to num fish
}

impl School {
    fn step(&mut self) {
        let mut new = [0; 9];
        for t in 0..9 {
            let n = self.counts[t];
            if t == 0 {
                new[8] += n;
                new[6] += n;
            } else {
                new[t-1] += n;
            }
        }
        self.counts = new;
    }
}
