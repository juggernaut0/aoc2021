use std::collections::HashSet;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let (algo, mut image) = parse_input(&input);

       image.enhance(&algo);
       image.enhance(&algo);

       let lit = image.grid.len();

       format!("{}", lit)
   }

   fn solve_2(&self, input: String) -> String {
       let (algo, mut image) = parse_input(&input);

       for _ in 0..50 {
           image.enhance(&algo);
       }

       let lit = image.grid.len();

       format!("{}", lit)
   }
}

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let mut lines = input.lines();
    let algo = lines.next().unwrap().chars().map(|c| c == '#').collect();
    lines.next().unwrap();
    let mut grid = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(Point(x as i32, y as i32));
            }
        }
    }
    (algo, Image { grid, lit: true })
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

struct Image {
    grid: HashSet<Point>,
    lit: bool, // if true, grid contains lit points, otherwise it contains unlit points
}

impl Image {
    fn get(&self, p: Point) -> bool {
        if self.lit {
            self.grid.contains(&p)
        } else {
            !self.grid.contains(&p)
        }
    }

    fn enhance(&mut self, algo: &[bool]) {
        let mut new_grid = HashSet::new();
        let new_lit = if algo[0] { !self.lit } else { self.lit };
        let minx = self.grid.iter().map(|Point(x, _)| *x).min().unwrap() - 1;
        let maxx = self.grid.iter().map(|Point(x, _)| *x).max().unwrap() + 1;
        let miny = self.grid.iter().map(|Point(_, y)| *y).min().unwrap() - 1;
        let maxy = self.grid.iter().map(|Point(_, y)| *y).max().unwrap() + 1;
        for x in minx..=maxx {
            for y in miny..=maxy {
                let p = Point(x, y);
                let n = self.p_to_i(p);
                if algo[n] == new_lit {
                    new_grid.insert(p);
                }
            }
        }
        self.grid = new_grid;
        self.lit = new_lit;
    }

    fn p_to_i(&self, p: Point) -> usize {
        let mut n: usize = 0;
        for y in -1..2 {
            for x in -1..2 {
                n <<= 1;
                if self.get(Point(p.0 + x, p.1 + y)) {
                    n += 1;
                }
            }
        }
        log::debug!("{:?} final {}", p, n);
        n
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn test_p_to_i() {
        let mut grid = HashSet::new();
        grid.insert(Point(0, 0));
        grid.insert(Point(0, 1));
        grid.insert(Point(1, 0));
        grid.insert(Point(1, 1));
        grid.insert(Point(1, 2));
        grid.insert(Point(2, 1));
        let image = Image { grid, lit: true };
        assert_eq!(0b110111010, image.p_to_i(Point(1, 1)));
    }

    #[test]
    fn test_p_to_i_unlit() {
        let mut grid = HashSet::new();
        grid.insert(Point(0, 0));
        grid.insert(Point(0, 1));
        grid.insert(Point(1, 0));
        grid.insert(Point(1, 1));
        grid.insert(Point(1, 2));
        grid.insert(Point(2, 1));
        let image = Image { grid, lit: false };
        assert_eq!(0b001000101, image.p_to_i(Point(1, 1)));
    }

    #[test]
    fn pt1_ex() {
        let inp = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
".to_string();
        assert_eq!("35", Solution.solve_1(inp));
    }
}