use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let steps: Vec<Cuboid> = parse_lines(&input).filter(|it: &Cuboid| it.is_init()).collect();
       solve(&steps)
   }

   fn solve_2(&self, input: String) -> String {
       let steps: Vec<Cuboid> = parse_lines(&input).collect();
       solve(&steps)
   }
}

fn solve(steps: &[Cuboid]) -> String {
    let mut count: u64 = 0;
    for (i, step) in steps.iter().enumerate().filter(|(_, step)| step.on) {
        count += step.subtract_points(&steps[i+1..]);
    }

    format!("{}", count)
}

#[derive(Debug)]
struct Cuboid {
    on: bool,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cuboid {
    fn is_init(&self) -> bool {
        *self.x.start() >= -50 && *self.x.end() <= 50 &&
            *self.y.start() >= -50 && *self.y.end() <= 50 &&
            *self.z.start() >= -50 && *self.z.end() <= 50
    }

    fn volume(&self) -> u64 {
        let dx = (self.x.end() - self.x.start() + 1) as u64;
        let dy = (self.y.end() - self.y.start() + 1) as u64;
        let dz = (self.z.end() - self.z.start() + 1) as u64;
        dx * dy * dz
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let x = range_overlap(self.x.clone(), other.x.clone())?;
        let y = range_overlap(self.y.clone(), other.y.clone())?;
        let z = range_overlap(self.z.clone(), other.z.clone())?;
        Some(Cuboid { on: true, x, y, z })
    }

    // others is the cuboids that come after self in the list of steps
    // returns the number of points where self is the last step to contain the point
    fn subtract_points(&self, others: &[Cuboid]) -> u64 {
        let mut p = self.volume();
        for (i, other) in others.iter().enumerate() {
            // int is the intersection of self and some cuboid that comes after self
            if let Some(int) = self.intersect(other) {
                // remove only the points where the intersection is the last cuboid
                // because future iterations of this loop will remove more
                p -= int.subtract_points(&others[i+1..])
            }
        }
        p
    }
}

fn range_overlap<T: Ord + Copy>(r1: RangeInclusive<T>, r2: RangeInclusive<T>) -> Option<RangeInclusive<T>> {
    if r1.end() <= r2.start() || r2.end() <= r1.start() { // |--r1--|  |--r2--| or |--r2--|  |--r1--|
        None
    } else if r2.contains(r1.start()) && r2.contains(r1.end()) { // |--r2-|--r1--|-|
        Some(r1)
    } else if r1.contains(r2.start()) {
        let x1 = *r2.start();
        let x2 = if r1.contains(r2.end()) { // |--r1-|--r2--|-|
            *r2.end()
        } else { // |--r1-|--|-r2--|
            *r1.end()
        };
        Some(x1..=x2)
    } else if r1.start() > r2.start() { // |--r2-|--|-r1--|
        let x1 = *r1.start();
        let x2 = *r2.end();
        Some(x1..=x2)
    } else {
        None
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let on = s.starts_with("on");
        let s = s.replace(|c: char| !(c == '-' || c.is_ascii_digit()), " ");
        log::debug!("{}", s);
        let mut coords = s.split_ascii_whitespace();
        let x1 = coords.next().unwrap().parse().unwrap();
        let x2 = coords.next().unwrap().parse().unwrap();
        let y1 = coords.next().unwrap().parse().unwrap();
        let y2 = coords.next().unwrap().parse().unwrap();
        let z1 = coords.next().unwrap().parse().unwrap();
        let z2 = coords.next().unwrap().parse().unwrap();
        Ok(Cuboid { on, x: x1..=x2, y: y1..=y2, z: z1..=z2 })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(i32, i32, i32);

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn test_smol() {
        let inp = "\
on x=-25..22,y=-34..11,z=-38..12
off x=-24..-15,y=26..39,z=-18..-9
on x=-23..22,y=-31..20,z=-38..-20
".to_string();
        assert_eq!("120474", Solution.solve_1(inp))
    }

    #[test]
    fn sub1() {
        let steps = vec![
            Cuboid {
                on: true,
                x: 0..=4,
                y: 0..=4,
                z: 0..=4,
            }
        ];
        assert_eq!(125, steps[0].subtract_points(&steps[1..]))
    }

    #[test]
    fn sub2() {
        let steps = vec![
            Cuboid {
                on: true,
                x: 0..=4,
                y: 0..=4,
                z: 0..=4,
            },
            Cuboid {
                on: true,
                x: 0..=1,
                y: 0..=1,
                z: 0..=1,
            },
        ];
        assert_eq!(117, steps[0].subtract_points(&steps[1..]))
    }
}