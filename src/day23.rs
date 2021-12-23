use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       solve(&input, true)
   }

   fn solve_2(&self, input: String) -> String {
       solve(&input, false)
   }
}

fn solve(input: &str, part_1: bool) -> String {
    let start = parse_input(input, part_1);
    let mut q = BinaryHeap::new();
    let mut seent = HashMap::new();
    q.push(Reverse(start));
    while let Some(Reverse(state)) = q.pop() {
        log::debug!("\n{}\nenergy: {}\npath: {:?}", state, state.energy, state.path);
        if state.finished() {
            return format!("{}", state.energy)
        } else {
            if let Some(e) = seent.get(&state.slots).copied() {
                if e <= state.energy {
                    continue
                }
            }
            seent.insert(state.slots, state.energy);
            let moves = state.moves();
            q.extend(moves.into_iter().map(Reverse));
        }
    }
    panic!("no solution found")
}

fn parse_input(input: &str, part_1: bool) -> Amphipods {
    let mut lines = input.lines().skip(2);
    let top = lines.next().unwrap();
    let top1 = top.chars().nth(3).unwrap();
    let top2 = top.chars().nth(5).unwrap();
    let top3 = top.chars().nth(7).unwrap();
    let top4 = top.chars().nth(9).unwrap();
    let bot = lines.next().unwrap();
    let bot1 = bot.chars().nth(3).unwrap();
    let bot2 = bot.chars().nth(5).unwrap();
    let bot3 = bot.chars().nth(7).unwrap();
    let bot4 = bot.chars().nth(9).unwrap();
    Amphipods::new(top1, top2, top3, top4, bot1, bot2, bot3, bot4, part_1)
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);

/*
#############
#01.2.3.4.56#
###7#8#9#0###
  #1#2#3#4#
  #5#6#7#8#
  #9#0#1#2#
  #########
 */
const POINTS: [Point; 23] = [
    Point(0, 0),
    Point(1, 0),
    Point(3, 0),
    Point(5, 0),
    Point(7, 0),
    Point(9, 0),
    Point(10, 0),

    Point(2, 1),
    Point(4, 1),
    Point(6, 1),
    Point(8, 1),

    Point(2, 2),
    Point(4, 2),
    Point(6, 2),
    Point(8, 2),

    Point(2, 3),
    Point(4, 3),
    Point(6, 3),
    Point(8, 3),

    Point(2, 4),
    Point(4, 4),
    Point(6, 4),
    Point(8, 4),
];

struct Amphipods {
    slots: [Option<char>; 23],
    energy: u32,
    path: Vec<(usize, usize)>,
    part_1: bool,
}

impl Amphipods {
    #[allow(clippy::too_many_arguments)]
    fn new(
        top1: char, top2: char, top3: char, top4: char,
        bot1: char, bot2: char, bot3: char, bot4: char,
        part_1: bool,
    ) -> Amphipods {
        let slots = if part_1 {
            [
                None, None, None, None, None, None, None,
                Some(top1), Some(top2), Some(top3), Some(top4),
                Some(bot1), Some(bot2), Some(bot3), Some(bot4),
                Some('A'), Some('B'), Some('C'), Some('D'),
                Some('A'), Some('B'), Some('C'), Some('D'),
            ]
        } else {
            [
                None, None, None, None, None, None, None,
                Some(top1), Some(top2), Some(top3), Some(top4),
                Some('D'), Some('C'), Some('B'), Some('A'),
                Some('D'), Some('B'), Some('A'), Some('C'),
                Some(bot1), Some(bot2), Some(bot3), Some(bot4),
            ]
        };
        Amphipods {
            slots,
            energy: 0,
            path: Vec::new(),
            part_1,
        }
    }

    fn distance(from: usize, to: usize) -> u32 {
        let from_p = POINTS[from];
        let to_p = POINTS[to];
        ((from_p.0 - to_p.0).abs() + (from_p.1 - to_p.1).abs()) as u32
    }

    fn finished(&self) -> bool {
        self.slots == [
            None, None, None, None, None, None, None,
            Some('A'), Some('B'), Some('C'), Some('D'),
            Some('A'), Some('B'), Some('C'), Some('D'),
            Some('A'), Some('B'), Some('C'), Some('D'),
            Some('A'), Some('B'), Some('C'), Some('D'),
        ]
    }

    fn is_correct_room(&self, from: usize, exp: char) -> bool {
        if from < 7 {
            return false
        }
        let self_correct = self.slots[from] == Some(exp);
        if from + 4 > 22 {
            self_correct
        } else {
            self_correct && self.is_correct_room(from + 4, exp)
        }
    }

    fn make_move(&self, from: usize, to: usize) -> Option<Amphipods> {
        // if part one you can't move past 14
        if self.part_1 && (to > 14 || from > 14) {
            return None
        }
        // can't move nothing
        let c = self.slots[from]?;
        // can't move to occupied spot
        if self.slots[to].is_some() {
            return None
        }

        let from_p = POINTS[from];
        let to_p = POINTS[to];
        log::trace!("trying to move from {} to {}", from, to);

        // if you're in the right spot already, don't move
        if [7, 11, 15, 19].contains(&from) && self.is_correct_room(from, 'A') {
            log::trace!("was in the right spot");
            return None
        }
        if [8, 12, 16, 20].contains(&from) && self.is_correct_room(from, 'B') {
            log::trace!("was in the right spot");
            return None
        }
        if [9, 13, 17, 21].contains(&from) && self.is_correct_room(from, 'C') {
            log::trace!("was in the right spot");
            return None
        }
        if [10, 14, 18, 22].contains(&from) && self.is_correct_room(from, 'D') {
            log::trace!("was in the right spot");
            return None
        }

        // one in the lower room slot can't move if same room upper slot is occupied
        if (11..23).contains(&from) {
            let mut i = from - 4;
            while i > 6 {
                if self.slots[i].is_some() {
                    log::trace!("was blocked trying to move up");
                    return None
                }
                i -= 4;
            }
        }

        // can't move past another pod in the hall
        {
            let from_x = from_p.0;
            let to_x = to_p.0;
            let range = if from_x < to_x { from_x+1..to_x+1 } else { to_x..from_x };
            for (p, s) in POINTS.iter().zip(&self.slots).take(6) {
                if range.contains(&p.0) && s.is_some() {
                    log::trace!("was blocked tring to move over");
                    return None
                }
            }
        }

        // from a room you can only move into the hallway
        if (7..23).contains(&from) && (7..23).contains(&to) {
            log::trace!("was trying to move room-to-room");
            return None
        }
        // from the hallway you can only move into a room
        if (0..=6).contains(&from) && (0..=6).contains(&to) {
            log::trace!("was trying to move hall-to-hall");
            return None
        }

        // Can't move to non-final rooms
        if to > 6 {
            if c == 'A' && to_p.0 != 2 {
                log::trace!("was trying to move to non-final room");
                return None
            }
            if c == 'B' && to_p.0 != 4 {
                log::trace!("was trying to move to non-final room");
                return None
            }
            if c == 'C' && to_p.0 != 6 {
                log::trace!("was trying to move to non-final room");
                return None
            }
            if c == 'D' && to_p.0 != 8 {
                log::trace!("was trying to move to non-final room");
                return None
            }
        }

        // can't move to top of room if bottom is not matching
        if (7..23).contains(&to) {
            let mut i = to + 4;
            while i < 23 {
                if self.slots[i] != Some(c) {
                    log::trace!("was trying to move to incomplete room");
                    return None
                }
                i += 4;
            }
        }

        let eng_mul = match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("! At the amphipods: {:?}", c)
        };
        let new_eng = self.energy + Self::distance(from, to) * eng_mul;
        let mut new_slots = self.slots;
        new_slots[from] = None;
        new_slots[to] = Some(c);
        let mut new_path = self.path.clone();
        new_path.push((from, to));
        Some(Amphipods { slots: new_slots, energy: new_eng, path: new_path, part_1: self.part_1 })
    }

    fn moves(self) -> Vec<Amphipods> {
        let mut res = Vec::new();
        for from in 0..23 {
            for to in 0..23 {
                if from != to {
                    if let Some(mv) = self.make_move(from, to) {
                        res.push(mv);
                    }
                }
            }
        }
        res
    }
}

impl PartialEq for Amphipods {
    fn eq(&self, other: &Self) -> bool {
        self.energy == other.energy
    }
}
impl Eq for Amphipods {}

impl PartialOrd for Amphipods {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Amphipods {
    fn cmp(&self, other: &Self) -> Ordering {
        self.energy.cmp(&other.energy)
    }
}

impl Display for Amphipods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
#############
#{}{}.{}.{}.{}.{}{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #########
",
               self.slots[0].unwrap_or('.'),
               self.slots[1].unwrap_or('.'),
               self.slots[2].unwrap_or('.'),
               self.slots[3].unwrap_or('.'),
               self.slots[4].unwrap_or('.'),
               self.slots[5].unwrap_or('.'),
               self.slots[6].unwrap_or('.'),
               self.slots[7].unwrap_or('.'),
               self.slots[8].unwrap_or('.'),
               self.slots[9].unwrap_or('.'),
               self.slots[10].unwrap_or('.'),
               self.slots[11].unwrap_or('.'),
               self.slots[12].unwrap_or('.'),
               self.slots[13].unwrap_or('.'),
               self.slots[14].unwrap_or('.'),
               self.slots[15].unwrap_or('.'),
               self.slots[16].unwrap_or('.'),
               self.slots[17].unwrap_or('.'),
               self.slots[18].unwrap_or('.'),
               self.slots[19].unwrap_or('.'),
               self.slots[20].unwrap_or('.'),
               self.slots[21].unwrap_or('.'),
               self.slots[22].unwrap_or('.'),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use super::*;

    #[test]
    fn test_finished() {
        let a = Amphipods::new('A', 'B', 'C', 'D', 'A', 'B', 'C', 'D', true);
        assert!(a.finished())
    }

    #[test]
    fn ex_pt1() {
        crate::init_test_logging();
        let inp = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########".to_string();
        assert_eq!("12521", Solution.solve_1(inp));
    }

    #[test]
    fn step1() {
        crate::init_test_logging();
        /*
#############
#.B.C...C.A.#
###D#.#.#D###
  #D#.#.#A#
  #D#B#A#C#
  #B#C#B#A#
  #########
         */
        let slots = [
            None, Some('B'), Some('C'), None, Some('C'), Some('A'), None,
            Some('D'), None, None, Some('D'),
            Some('D'), None, None, Some('A'),
            Some('D'), Some('B'), Some('A'), Some('C'),
            Some('B'), Some('C'), Some('B'), Some('A'),
        ];
        let pods = Amphipods { slots, energy: 0, path: Vec::new(), part_1: false };
        let moves = pods.moves();
        assert!(moves.len() > 0)
    }
}