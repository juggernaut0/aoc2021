use std::collections::HashSet;
use std::str::FromStr;
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let (points, folds) = parse_input(&input);

       let result = apply_fold(points, folds.into_iter().next().unwrap());

       format!("{}", result.len())
   }

   fn solve_2(&self, input: String) -> String {
       let (points, folds) = parse_input(&input);

       let result = folds.into_iter().fold(points, apply_fold);

       display_points(&result);

       String::new()
   }
}

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut parts = input.split("\n\n");
    let points_raw = parts.next().unwrap();
    let points = parse_lines(points_raw).collect();

    let folds_raw = parts.next().unwrap();
    let folds = parse_lines(folds_raw).collect();

    (points, folds)
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|it| it.parse().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        Ok(Point(x, y))
    }
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    n: i32,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let juicy = s.split_whitespace().last().unwrap();
        let mut parts = juicy.split('=');
        let axis = match parts.next().unwrap() {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => unreachable!()
        };
        let n = parts.next().unwrap().parse().unwrap();
        Ok(Fold { axis, n })
    }
}

fn apply_fold(points: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    log::debug!("folding {:?}", fold);

    let mut res = HashSet::new();

    for point in points {
        let p = match fold.axis {
            Axis::X => {
                if point.0 == fold.n { panic!("fold on dot") }
                let new_x = if point.0 < fold.n {
                    point.0
                } else {
                    2*fold.n - point.0
                };
                Point(new_x, point.1)
            }
            Axis::Y => {
                if point.1 == fold.n { panic!("fold on dot") }
                let new_y = if point.1 < fold.n {
                    point.1
                } else {
                    2*fold.n - point.1
                };
                Point(point.0, new_y)
            }
        };
        log::debug!("{:?} -> {:?}", point, p);
        res.insert(p);
    }

    res
}

fn display_points(points: &HashSet<Point>) {
    let max_y = points.iter().map(|Point(_, y)| *y).max().unwrap();
    for y in 0..=max_y {
        let max_x = points.iter()
            .filter_map(|Point(px, py)| {
                if y == *py {
                    Some(*px)
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        for x in 0..=max_x {
            if points.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn pt1_ex() {
        crate::init_test_logging();
        let inp = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5".to_string();
        assert_eq!("17", Solution.solve_1(inp));
    }
}
