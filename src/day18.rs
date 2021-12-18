use std::iter::Peekable;
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::str::{Chars, FromStr};
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let res: SfNum = parse_lines(&input).reduce(SfNum::add).unwrap();

       format!("{}", res.magnitude())
   }

   fn solve_2(&self, input: String) -> String {
       let nums: Vec<SfNum> = parse_lines(&input).collect();

       let res = nums.iter()
           .flat_map(|a| nums.iter().map(move |b| (a, b)))
           .map(|(a, b)| SfNum::add(a.clone(), b.clone()).magnitude())
           .max()
           .unwrap();

       format!("{}", res)
   }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum SfNum {
    Regular(i32),
    Pair(Box<SfNum>, Box<SfNum>),
}

impl SfNum {
    fn into_pair(self) -> (SfNum, SfNum) {
        match self {
            SfNum::Regular(_) => panic!("not a pair"),
            SfNum::Pair(a, b) => (*a, *b)
        }
    }

    fn to_reg(&self) -> i32 {
        match self {
            SfNum::Regular(n) => *n,
            SfNum::Pair(_, _) => panic!("not a regular number")
        }
    }

    fn as_reg_mut(&mut self) -> &mut i32 {
        match self {
            SfNum::Regular(n) => n,
            SfNum::Pair(_, _) => panic!("not a regular number")
        }
    }

    fn add(a: SfNum, b: SfNum) -> SfNum {
        let mut raw = SfNum::Pair(Box::new(a), Box::new(b));
        raw.reduce();
        raw
    }

    fn reduce(&mut self) {
        log::debug!("reducing {:?}", self);
        loop {
            match reduce_step(self) {
                Continue(()) => continue,
                Break(()) => break,
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            SfNum::Regular(n) => *n,
            SfNum::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn navigate(&self, path: &[PathDir]) -> &SfNum {
        if path.is_empty() {
            self
        } else if let SfNum::Pair(left, right) = self {
            let (now, later) = path.split_first().unwrap();
            if *now == PathDir::Left {
                left.navigate(later)
            } else {
                right.navigate(later)
            }
        } else {
            panic!("invalid path")
        }
    }

    fn navigate_mut(&mut self, path: &[PathDir]) -> &mut SfNum {
        if path.is_empty() {
            self
        } else if let SfNum::Pair(left, right) = self {
            let (now, later) = path.split_first().unwrap();
            if *now == PathDir::Left {
                left.navigate_mut(later)
            } else {
                right.navigate_mut(later)
            }
        } else {
            panic!("invalid path")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PathDir { Left, Right }

fn find_nested(num: &SfNum, mut path: Vec<PathDir>) -> Option<Vec<PathDir>> {
    if path.len() == 4 {
        if let SfNum::Pair(_, _) = num {
            Some(path)
        } else {
            None
        }
    } else if let SfNum::Pair(left, right) = num {
        {
            let mut path = path.clone();
            path.push(PathDir::Left);
            find_nested(left, path)
        }.or_else(move || {
            path.push(PathDir::Right);
            find_nested(right, path)
        })
    } else {
        None
    }
}

fn find_left_cousin(num: &SfNum, target: &[PathDir]) -> Option<Vec<PathDir>> {
    let mut res: Vec<_> = target.iter().copied().collect();
    loop {
        match res.pop() {
            None => return None,
            Some(PathDir::Left) => continue,
            Some(PathDir::Right) => break,
        }
    }
    res.push(PathDir::Left);
    loop {
        match num.navigate(&res) {
            SfNum::Regular(_) => break,
            SfNum::Pair(_, _) => {
                res.push(PathDir::Right);
            }
        }
    }
    Some(res)
}

fn find_right_cousin(num: &SfNum, target: &[PathDir]) -> Option<Vec<PathDir>> {
    let mut res: Vec<_> = target.iter().copied().collect();
    loop {
        match res.pop() {
            None => return None,
            Some(PathDir::Right) => continue,
            Some(PathDir::Left) => break,
        }
    }
    res.push(PathDir::Right);
    loop {
        match num.navigate(&res) {
            SfNum::Regular(_) => break,
            SfNum::Pair(_, _) => {
                res.push(PathDir::Left);
            }
        }
    }
    Some(res)
}

fn find_large_reg(num: &SfNum, mut path: Vec<PathDir>) -> Option<Vec<PathDir>> {
    match num {
        SfNum::Regular(n) if *n >= 10 => Some(path),
        SfNum::Regular(_) => None,
        SfNum::Pair(left, right) => {
            {
                let mut path = path.clone();
                path.push(PathDir::Left);
                find_large_reg(left, path)
            }.or_else(move || {
                path.push(PathDir::Right);
                find_large_reg(right, path)
            })
        }
    }
}

fn reduce_step(num: &mut SfNum) -> ControlFlow<()> {
    log::debug!("reduce step {:?}", num);
    if let Some(path) = find_nested(&num, Vec::new()) {
        let target = num.navigate_mut(&path);
        let old = std::mem::replace(target, SfNum::Regular(0));
        let (a, b) = old.into_pair();
        if let Some(lc) = find_left_cousin(&num, &path) {
            *num.navigate_mut(&lc).as_reg_mut() += a.to_reg();
        }
        if let Some(rc) = find_right_cousin(&num, &path) {
            *num.navigate_mut(&rc).as_reg_mut() += b.to_reg();
        }
        Continue(())
    } else if let Some(path) = find_large_reg(&num, Vec::new()) {
        let n = num.navigate(&path).to_reg();
        let a = n / 2;
        let b = n - a;
        //*num.navigate_mut(&path) = SfNum::add(SfNum::Regular(a), SfNum::Regular(b));
        *num.navigate_mut(&path) = SfNum::Pair(
            Box::new(SfNum::Regular(a)),
            Box::new(SfNum::Regular(b)),
        );
        Continue(())
    } else {
        Break(())
    }
}

impl FromStr for SfNum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Parser::new(s).parse())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SfToken {
    Number(i32),
    Comma,
    Open,
    Close,
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl Parser<'_> {
    fn new(s: &str) -> Parser {
        Parser {
            chars: s.chars().peekable(),
        }
    }

    fn next(&mut self) -> SfToken {
        if self.chars.next_if_eq(&'[').is_some() {
            SfToken::Open
        } else if self.chars.next_if_eq(&']').is_some() {
            SfToken::Close
        } else if self.chars.next_if_eq(&',').is_some() {
            SfToken::Comma
        } else {
            let mut s = String::new();
            while let Some(c) = self.chars.next_if(|c| c.is_ascii_digit()) {
                s.push(c);
            }
            SfToken::Number(s.parse().unwrap())
        }
    }

    fn parse(&mut self) -> SfNum {
        match self.next() {
            SfToken::Open => {
                let left = self.parse();
                assert_eq!(self.next(), SfToken::Comma);
                let right = self.parse();
                assert_eq!(self.next(), SfToken::Close);
                SfNum::Pair(Box::new(left), Box::new(right))
            }
            SfToken::Number(n) => {
                SfNum::Regular(n)
            }
            other => panic!("unknown token {:?}", other)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day18::PathDir::{Left, Right};
    use super::*;

    #[test]
    fn test_find_nested() {
        let num: SfNum = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        let path = find_nested(&num, Vec::new());
        assert_eq!(Some(vec![Left, Right, Right, Right]), path);
        let path = path.unwrap();
        let elem = num.navigate(&path);
        assert_eq!(&SfNum::Pair(Box::new(SfNum::Regular(7)), Box::new(SfNum::Regular(3))), elem);
        let left_cousin = find_left_cousin(&num, &path);
        assert_eq!(Some(vec![Left, Right, Right, Left]), left_cousin);
        let right_cousin = find_right_cousin(&num, &path);
        assert_eq!(Some(vec![Right, Left]), right_cousin);
    }

    #[test]
    fn test_reduce() {
        let mut num: SfNum = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        num.reduce();
        let exp: SfNum = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(exp, num);
    }
}