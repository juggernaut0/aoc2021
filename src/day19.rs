use std::collections::HashSet;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let mut scanners = parse_input(&input);

       // assuming every scanner shares 12 points with at least one other scanner

       let mut found: HashSet<_> = scanners.pop().unwrap().into_iter().collect();
       while !scanners.is_empty() {
           let mut to_remove = None;
           for (si, s) in scanners.iter().enumerate() {
               if let Some((or, tr)) = find_scanner_pos(&found, s, 12) {
                   log::info!("found a match");
                   for p in s {
                       found.insert(p.rotate(or).translate(tr));
                   }
                   to_remove = Some(si);
                   break
               }
           }
           scanners.remove(to_remove.unwrap());
       }

       format!("{}", found.len())
   }

   fn solve_2(&self, input: String) -> String {
       let mut scanners = parse_input(&input);

       let mut found: HashSet<_> = scanners.pop().unwrap().into_iter().collect();
       let mut scanner_locs: Vec<Point> = Vec::new();
       while !scanners.is_empty() {
           let mut to_remove = None;
           for (si, s) in scanners.iter().enumerate() {
               if let Some((or, tr)) = find_scanner_pos(&found, s, 12) {
                   log::info!("found a match");
                   scanner_locs.push(tr);
                   for p in s {
                       found.insert(p.rotate(or).translate(tr));
                   }
                   to_remove = Some(si);
                   break
               }
           }
           scanners.remove(to_remove.unwrap());
       }

       let mut max = 0;
       for sl1 in &scanner_locs {
           for sl2 in &scanner_locs {
               let d = (sl1.0 - sl2.0).abs() + (sl1.1 - sl2.1).abs() + (sl1.2 - sl2.2).abs();
               if d > max {
                   max = d;
               }
           }
       }

       format!("{}", max)
   }
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input.split("\n\n")
        .map(|scanner| {
            scanner.lines().skip(1)
                .map(|line| {
                    let mut coords = line.split(',').map(|c| c.parse::<i32>().unwrap());
                    Point(coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap())
                })
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(i32, i32, i32);

impl Point {
    fn rotate(self, i: usize) -> Point {
        let Point(x, y, z) = self;
        match i {
             0 => Point(x, y, z),
             1 => Point(x, -y, -z),
             2 => Point(x, z, -y),
             3 => Point(x, -z, y),
             4 => Point(-x, y, -z),
             5 => Point(-x, -y, z),
             6 => Point(-x, z, y),
             7 => Point(-x, -z, -y),
             8 => Point(y, z, x),
             9 => Point(y, -z, -x),
            10 => Point(y, x, -z),
            11 => Point(y, -x, z),
            12 => Point(-y, z, -x),
            13 => Point(-y, -z, x),
            14 => Point(-y, x, z),
            15 => Point(-y, -x, -z),
            16 => Point(z, x, y),
            17 => Point(z, -x, -y),
            18 => Point(z, y, -x),
            19 => Point(z, -y, x),
            20 => Point(-z, x, -y),
            21 => Point(-z, -x, y),
            22 => Point(-z, y, x),
            23 => Point(-z, -y, -x),
            _ => panic!("bad rot")
        }
    }

    fn translate(self, Point(dx, dy, dz): Point) -> Point {
        let Point(x, y, z) = self;
        Point(x + dx, y + dy, z + dz)
    }
}

// returns the position and orientation of scanner s2 relative to s1 if the boxes overlap
fn find_scanner_pos(s1: &HashSet<Point>, s2: &[Point], req_matches: usize) -> Option<(usize, Point)> {
    for i in 0..24 {
        let s2_rot: Vec<_> = s2.iter().copied().map(|p| p.rotate(i)).collect();
        for s2_orig in &s2_rot {
            for s1_orig in s1 {
                let dx = s1_orig.0 - s2_orig.0;
                let dy = s1_orig.1 - s2_orig.1;
                let dz = s1_orig.2 - s2_orig.2;
                let tr = Point(dx, dy, dz);

                let same_count = s2_rot.iter()
                    .map(|p| p.translate(tr))
                    .filter(|p| s1.contains(p))
                    .take(12)
                    .count();
                if same_count >= req_matches {
                    return Some((i, tr));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn rotate_scanner(s: &mut [Point], i: usize) {
        for p in s {
            *p = p.rotate(i);
        }
    }

    fn translate_scanner(s: &mut [Point], t: Point) {
        for p in s {
            *p = p.translate(t);
        }
    }

    #[test]
    fn test_small() {
        crate::init_test_logging();
        let s0 = vec![
            Point(1, 2, 3),
            Point(4, 5, 6),
            Point(7, 6, 5),
        ].into_iter().collect();
        let s1 = vec![
            Point(-6, 7, -8),
            Point(-3, 4, -5),
            Point(-9, 8, -7),
        ];
        assert_eq!(Some((4, Point(-2, -2, -2))), find_scanner_pos(&s0, &s1, 3))
    }

    #[test]
    fn test_rotate() {
        crate::init_test_logging();
        let s0 = vec![
            Point(404,-588,-901),
            Point(528,-643,409),
            Point(-838,591,734),
            Point(390,-675,-793),
            Point(-537,-823,-458),
            Point(-485,-357,347),
            Point(-345,-311,381),
            Point(-661,-816,-575),
            Point(-876,649,763),
            Point(-618,-824,-621),
            Point(553,345,-567),
            Point(474,580,667),
            Point(-447,-329,318),
            Point(-584,868,-557),
            Point(544,-627,-890),
            Point(564,392,-477),
            Point(455,729,728),
            Point(-892,524,684),
            Point(-689,845,-530),
            Point(423,-701,434),
            Point(7,-33,-71),
            Point(630,319,-379),
            Point(443,580,662),
            Point(-789,900,-551),
            Point(459,-707,401),
        ].into_iter().collect();
        let mut s1 = vec![
            Point(686,422,578),
            Point(605,423,415),
            Point(515,917,-361),
            Point(-336,658,858),
            Point(95,138,22),
            Point(-476,619,847),
            Point(-340,-569,-846),
            Point(567,-361,727),
            Point(-460,603,-452),
            Point(669,-402,600),
            Point(729,430,532),
            Point(-500,-761,534),
            Point(-322,571,750),
            Point(-466,-666,-811),
            Point(-429,-592,574),
            Point(-355,545,-477),
            Point(703,-491,-529),
            Point(-328,-685,520),
            Point(413,935,-424),
            Point(-391,539,-444),
            Point(586,-435,557),
            Point(-364,-763,-893),
            Point(807,-499,-711),
            Point(755,-354,-619),
            Point(553,889,-390),
        ];

        let (oi, tr) = find_scanner_pos(&s0, &s1, 12).unwrap();
        log::debug!("{} {:?}", oi, tr);
        rotate_scanner(&mut s1, oi);
        translate_scanner(&mut s1, tr);

        let common = vec![
            Point(-618,-824,-621),
            Point(-537,-823,-458),
            Point(-447,-329,318),
            Point(404,-588,-901),
            Point(544,-627,-890),
            Point(528,-643,409),
            Point(-661,-816,-575),
            Point(390,-675,-793),
            Point(423,-701,434),
            Point(-345,-311,381),
            Point(459,-707,401),
            Point(-485,-357,347),
        ];

        assert!(common.iter().all(|cp| s0.contains(cp) && s1.contains(cp)))
    }
}