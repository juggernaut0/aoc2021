use std::cmp::Ordering;
use crate::util::parse_lines_with;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let lines: Vec<BitStr> = parse_lines_with(&input, |s| BitStr::new(s).unwrap()).collect();
       let last = lines[0].len();

       let mut res = String::new();
       for i in 0..last {
           if most_common(&lines, i).unwrap() {
               res.push('1')
           } else {
               res.push('0')
           }
       }
       let g = BitStr::new(&res).unwrap().to_u32();
       let e = !g & !(u32::MAX << last);
       log::info!("g = {} e = {}", g, e);
       (e * g).to_string()
   }

   fn solve_2(&self, input: String) -> String {
       let lines: Vec<BitStr> = parse_lines_with(&input, |s| BitStr::new(s).unwrap()).collect();
       let last = lines[0].len();

       let a = {
           let mut remaining: Vec<_> = lines.clone();
           for i in 0..last {
               if remaining.len() == 1 { break; }
               let cond = most_common(&remaining, i).unwrap_or(true);
               remaining.retain(|l| l.get(i) == cond);
           }
           remaining[0].to_u32()
       };
       log::info!("{}", a);

       let b = {
           let mut remaining: Vec<_> = lines;
           for i in 0..last {
               if remaining.len() == 1 { break; }
               let cond = !most_common(&remaining, i).unwrap_or(true);
               remaining.retain(|l| l.get(i) == cond);
           }
           remaining[0].to_u32()
       };
       log::info!("{}", b);

       (a*b).to_string()
   }
}

#[derive(Copy, Clone)]
struct BitStr<'a>(&'a str);

impl BitStr<'_> {
    fn new(s: &str) -> Option<BitStr> {
        if s.chars().all(|c| c == '0' || c == '1') {
            Some(BitStr(s))
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn to_u32(self) -> u32 {
        u32::from_str_radix(self.0, 2).unwrap()
    }

    fn get(&self, index: usize) -> bool {
        self.0.chars().nth(index).unwrap() == '1'
    }
}

fn most_common(v: &[BitStr], i: usize) -> Option<bool> {
    let count1 = v.iter().filter(|l| l.get(i)).count();
    match (count1*2).cmp(&v.len()) {
        Ordering::Less => Some(false),
        Ordering::Equal => None,
        Ordering::Greater => Some(true),
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;
    #[test]
    fn test_2() {
        crate::init_test_logging();
        let inp = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
".to_string();
        assert_eq!(&Solution.solve_2(inp), "230");
    }
}