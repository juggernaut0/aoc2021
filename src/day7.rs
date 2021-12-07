pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       solve(&input, cost_to_pos)
   }

   fn solve_2(&self, input: String) -> String {
       solve(&input, cost_to_pos_2)
   }
}

fn solve(input: &str, cost_fn: impl Fn(&[i32], i32) -> i32) -> String {
    let poss: Vec<i32> = input.split(',').map(|it| it.trim().parse().unwrap()).collect();

    let min = poss.iter().copied().min().unwrap();
    let max = poss.iter().copied().max().unwrap();

    let res = (min..=max).map(|t| cost_fn(&poss, t)).min().unwrap();

    format!("{}", res)
}

fn cost_to_pos(starts: &[i32], target: i32) -> i32 {
    starts.iter().copied().map(|s| (s - target).abs()).sum()
}

fn cost_to_pos_2(starts: &[i32], target: i32) -> i32 {
    starts.iter().copied().map(|s| {
        let d = (s - target).abs();
        d*(d+1)/2
    }).sum()
}
