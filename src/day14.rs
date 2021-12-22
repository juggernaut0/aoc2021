use std::collections::HashMap;
use std::str::FromStr;
use crate::util::Counter;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       solve(&input, 10)
   }

   fn solve_2(&self, input: String) -> String {
       solve(&input, 40)
   }
}

fn solve(input: &str, times: i32) -> String {
    let (template, rules) = parse_input(input);

    let mut pair_counts = Counter::new();
    for window in template.windows(2) {
        pair_counts.count((window[0], window[1]));
    }

    let res = (0..times)
        .fold(pair_counts, |pairs, _| apply_rules(pairs, &rules));
    let mut counts: HashMap<char, u64> = HashMap::new();
    for ((a, _), c) in res {
        *counts.entry(a).or_default() += c;
    }
    *counts.entry(*template.last().unwrap()).or_default() += 1;

    let max = *counts.values().max().unwrap();
    let min = *counts.values().min().unwrap();

    format!("{}", max - min)
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Rule>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let rules = lines.map(|line| line.parse().unwrap()).collect();
    (template, rules)
}

struct Rule {
    in1: char,
    in2: char,
    out: char,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let mut ins = parts.next().unwrap().chars();
        let in1 = ins.next().unwrap();
        let in2 = ins.next().unwrap();
        let out = parts.next().unwrap().chars().next().unwrap();
        Ok(Rule { in1, in2, out })
    }
}

fn apply_rules(pairs: Counter<(char, char)>, rules: &[Rule]) -> Counter<(char, char)> {
    log::debug!("{:?}", pairs);

    let mut res = Counter::new();

    for ((a, b), count) in pairs {
        if let Some(rule) = rules.iter().find(|rule| rule.in1 == a && rule.in2 == b) {
            let c = rule.out;
            log::debug!("adding a {}{} and {}{} from {}{} -> {}", a, c, c, b, a, b, c);
            res.count_n((a, c), count);
            res.count_n((c, b), count);
        } else {
            log::debug!("rule not found for {}{}", a, b);
            res.count_n((a, b), count);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pt1_ex() {
        crate::init_test_logging();
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!("1588", solve(input, 10))
    }
}
