use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let score: u32 = input.lines()
           .filter_map(|line| {
               match parse_line(line) {
                   Ok(()) => panic!("ok line"),
                   Err(ParseError::Incomplete(_)) => None,
                   Err(ParseError::Corrupted(c)) => {
                       let cs = match c {
                           ')' => 3,
                           ']' => 57,
                           '}' => 1197,
                           '>' => 25137,
                           _ => unreachable!(),
                       };
                       Some(cs)
                   },
               }
           })
           .sum();

       format!("{}", score)
   }

   fn solve_2(&self, input: String) -> String {
       let mut scores: Vec<u64> = input.lines()
           .filter_map(|line| {
               match parse_line(line) {
                   Ok(()) => panic!("ok line"),
                   Err(ParseError::Incomplete(s)) => {
                       let score = s.chars().fold(0, |score, c| {
                           let cs = match c {
                               ')' => 1,
                               ']' => 2,
                               '}' => 3,
                               '>' => 4,
                               _ => unreachable!(),
                           };
                           score * 5 + cs
                       });
                       Some(score)
                   },
                   Err(ParseError::Corrupted(_)) => None,
               }
           })
           .collect();

       scores.sort_unstable();

       format!("{}", scores[scores.len() / 2])
   }
}

enum ParseError {
    Corrupted(char),
    Incomplete(String),
}

fn parse_line(line: &str) -> Result<(), ParseError> {
    log::debug!("parsing line {}", line);
    let mut parser = ChunkParser { chars: line.chars().peekable() };
    while let Some(()) = parser.parse()? {}
    Ok(())
}

struct ChunkParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl ChunkParser<'_> {
    fn parse(&mut self) -> Result<Option<()>, ParseError> {
        let c = if let Some(c) = self.chars.next() {
            c
        } else {
            return Ok(None)
        };

        log::trace!("starting parse of {}",c);

        let closing = match c {
            '(' => ')',
            '[' => ']',
            '<' => '>',
            '{' => '}',
            _ => unreachable!(),
        };

        loop {
            match self.chars.peek().copied() {
                Some(p) if p == closing => {
                    self.chars.next().unwrap();
                    break
                },
                Some(p) if is_opening(p) => {
                    match self.parse() {
                        Ok(_) => {}
                        Err(ParseError::Incomplete(mut s)) => {
                            s.push(closing);
                            return Err(ParseError::Incomplete(s));
                        }
                        Err(pe) => return Err(pe),
                    }
                },
                Some(p) => return Err(ParseError::Corrupted(p)),
                None => return Err(ParseError::Incomplete(closing.to_string())),
            }
        }

        log::trace!("parsing ok {}", c);

        Ok(Some(()))
    }
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '<' || c == '{'
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn test1() {
        crate::init_test_logging();

        let inp = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".to_string();

        assert_eq!("26397", &Solution.solve_1(inp));
    }
}
