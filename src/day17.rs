use lazy_static::lazy_static;
use regex::Regex;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let target = parse_input(&input);
       let mut res = 0;
       for vx in 1..50 {
           for vy in 0..500 {
               let traj = launch(Point(vx, vy));
               if traj.iter().any(|p| target.contains(*p)) {
                   log::debug!("{:?}", traj);
                   let max = traj.into_iter().map(|p| p.1).max().unwrap();
                   if max > res {
                       res = max;
                   }
               }
           }
       }

       format!("{}", res)
   }

   fn solve_2(&self, input: String) -> String {
       let target = parse_input(&input);
       let mut res = 0;
       for vx in 1..100 {
           for vy in -200..500 {
               let traj = launch(Point(vx, vy));
               if traj.iter().any(|p| target.contains(*p)) {
                   res += 1;
               }
           }
       }

       format!("{}", res)
   }
}

fn parse_input(input: &str) -> GridBox {
    // target area: x=70..96, y=-179..-124
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    }
    let caps = RE.captures(input.trim()).unwrap();
    let x1 = caps[1].parse().unwrap();
    let x2 = caps[2].parse().unwrap();
    let y1 = caps[3].parse().unwrap();
    let y2 = caps[4].parse().unwrap();
    GridBox {
        bottom_left: Point(x1, y1),
        top_right: Point(x2, y2),
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

struct GridBox {
    bottom_left: Point,
    top_right: Point,
}

impl GridBox {
    fn contains(&self, p: Point) -> bool {
        (self.bottom_left.0..=self.top_right.0).contains(&p.0)
            && (self.bottom_left.1..=self.top_right.1).contains(&p.1)
    }
}

fn launch(init_vel: Point) -> Vec<Point> {
    let mut probe = Probe { pos: Point(0, 0), vel: init_vel };
    let mut result = Vec::new();
    for _ in 0..500 {
        probe.step();
        result.push(probe.pos);
    }
    result
}

struct Probe {
    pos: Point,
    vel: Point,
}

impl Probe {
    fn step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.vel.0 -= self.vel.0.signum();
        self.vel.1 -= 1;
    }
}
