use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let mut lines = input.lines().peekable();
       let calls: Vec<u32> = lines.next().unwrap().split(',').map(|it| it.parse().unwrap()).collect();

       let mut boards = Vec::new();
       while lines.peek().is_some() {
           boards.push(parse_board(&mut lines));
       }

       for n in calls {
           log::debug!("calling {}", n);
           for board in &mut boards {
               board.mark(n);
               if board.is_winner() {
                   log::info!("{}", board);
                   return (n * board.score()).to_string()
               }
           }
       }

       unreachable!()
   }

   fn solve_2(&self, input: String) -> String {
       let mut lines = input.lines().peekable();
       let calls: Vec<u32> = lines.next().unwrap().split(',').map(|it| it.parse().unwrap()).collect();

       let mut boards = Vec::new();
       while lines.peek().is_some() {
           boards.push(parse_board(&mut lines));
       }

       let mut eliminated = HashSet::new();
       let mut last = 10000;
       for n in calls {
           log::debug!("calling {}", n);
           for (i, board) in boards.iter_mut().enumerate() {
               if eliminated.contains(&i) {
                   continue
               }
               board.mark(n);
               if board.is_winner() {
                   eliminated.insert(i);
                   last = i;
               }
           }
           if eliminated.len() == boards.len() {
               return format!("{}", boards[last].score() * n);
           }
       }

       unreachable!()
   }
}

struct BingoBoard {
    cells: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl BingoBoard {
    fn get_cell(&self, x: usize, y: usize) -> u32 {
        self.cells[x][y]
    }

    fn marked(&self, x: usize, y: usize) -> bool {
        self.marks[x][y]
    }

    fn mark(&mut self, n: u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.get_cell(x, y) == n {
                    self.marks[x][y] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        'xs: for x in 0..5 {
            for y in 0..5 {
                if !self.marked(x, y) {
                    continue 'xs
                }
            }
            return true
        }

        'ys: for y in 0..5 {
            for x in 0..5 {
                if !self.marked(x, y) {
                    continue 'ys
                }
            }
            return true
        }

        false
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.marked(x, y) {
                    sum += self.get_cell(x, y);
                }
            }
        }
        sum
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                if self.marked(x, y) {
                    write!(f, "{}* ", self.get_cell(x, y))?;
                } else {
                    write!(f, "{}  ", self.get_cell(x, y))?;
                }

            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_board<'a>(lines: &mut impl Iterator<Item=&'a str>) -> BingoBoard {
    let mut cells = [[0; 5]; 5];
    lines.next().unwrap(); // skip blank
    lines.take(5).enumerate().for_each(|(y, line)| {
        line.split_whitespace().enumerate().for_each(|(x, it)| {
            let n = it.parse().unwrap();
            cells[x][y] = n;
        });
    });

    BingoBoard { cells, marks: [[false; 5]; 5] }
}
