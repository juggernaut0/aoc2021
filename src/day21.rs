use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let (p1_pos, p2_pos) = parse_input(&input);
       let mut game = DiracDiceGame::new(DeterministicDice::new(), p1_pos, p2_pos, 1000);

       loop {
           if game.step() {
               break
           }
       }

       let state = game.state.iter().next().unwrap().0;
       let p1_score = state.p1.score;
       let p2_score = state.p2.score;
       log::info!("{} {} {}", p1_score, p2_score, game.dice.count);
       let res = min(p1_score, p2_score) * game.dice.count;
       format!("{}", res)
   }

   fn solve_2(&self, input: String) -> String {
       let (p1_pos, p2_pos) = parse_input(&input);
       let mut game = DiracDiceGame::new(DiracDice, p1_pos, p2_pos, 21);

       loop {
           if game.step() {
               break
           }
       }

       let res = game.wins.iter().max().copied().unwrap();
       format!("{}", res)
   }
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_ascii_whitespace().last().unwrap().parse().unwrap();
    let b = lines.next().unwrap().split_ascii_whitespace().last().unwrap().parse().unwrap();
    (a, b)
}

struct DiracDiceGame<T: Dice> {
    dice: T,
    state: Counter<GameState>,
    wins: [u64; 2],
    p1_turn: bool,
    score_limit: u32,
}

impl<T: Dice> DiracDiceGame<T> {
    fn new(dice: T, p1_pos: u32, p2_pos: u32, score_limit: u32) -> DiracDiceGame<T> {
        let mut state = Counter::new();
        state.count_n(GameState { p1: PlayerState::new(p1_pos), p2: PlayerState::new(p2_pos) }, 1);
        DiracDiceGame {
            dice,
            state,
            wins: [0, 0],
            p1_turn: true,
            score_limit,
        }
    }

    fn step(&mut self) -> bool {
        let rolls = self.dice.roll_3();
        let mut new_states = Counter::new();
        for (state, state_count) in self.state.iter() {
            let player = if self.p1_turn { state.p1 } else { state.p2 };
            for (roll, roll_count) in rolls.iter() {
                let mut new_player = player;
                let new_state_count = roll_count * state_count;
                new_player.advance(*roll);
                if new_player.score >= self.score_limit {
                    if self.p1_turn {
                        self.wins[0] += new_state_count;
                    } else {
                        self.wins[1] += new_state_count;
                    }
                } else {
                    let new_state = if self.p1_turn {
                        GameState {
                            p1: new_player,
                            p2: state.p2,
                        }
                    } else {
                        GameState {
                            p1: state.p1,
                            p2: new_player,
                        }
                    };
                    new_states.count_n(new_state, new_state_count);
                }
            }
        }
        if new_states.is_empty() {
            return true
        }
        self.state = new_states;
        self.p1_turn = !self.p1_turn;

        false
    }
}

#[derive(Hash, Eq, PartialEq)]
struct GameState {
    p1: PlayerState,
    p2: PlayerState,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct PlayerState {
    pos: u32,
    score: u32,
}

impl PlayerState {
    fn new(pos: u32) -> PlayerState {
        PlayerState {
            pos,
            score: 0
        }
    }

    fn advance(&mut self, roll: u32) -> u32 {
        let new_pos = (self.pos + roll - 1) % 10 + 1;
        self.pos = new_pos;
        self.score += new_pos;
        self.score
    }
}

trait Dice {
    fn roll(&mut self) -> Vec<u32>;

    fn roll_3(&mut self) -> Counter<u32> {
        let r1s = self.roll();
        let r2s = self.roll();
        let r3s = self.roll();
        let mut res = Counter::new();
        for r1 in r1s {
            for r2 in &r2s {
                for r3 in &r3s {
                    res.count_n(r1 + r2 + r3, 1);
                }
            }
        }
        res
    }
}

struct DeterministicDice {
    count: u32
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice { count: 0 }
    }
}

impl Dice for DeterministicDice {
    fn roll(&mut self) -> Vec<u32> {
        let r = self.count % 100 + 1;
        self.count += 1;
        vec![r]
    }
}

struct DiracDice;

impl Dice for DiracDice {
    fn roll(&mut self) -> Vec<u32> {
        vec![1, 2, 3]
    }
}

struct Counter<T, C = u64> {
    counts: HashMap<T, C>,
}

impl<T: Hash + Eq, C: Add<Output=C> + Default + Copy + Eq> Counter<T, C> {
    fn new() -> Counter<T, C> {
        Counter {
            counts: HashMap::new(),
        }
    }

    fn count_n(&mut self, k: T, n: C) -> C {
        let entry = self.counts.entry(k).or_default();
        *entry = *entry + n;
        *entry
    }

    fn iter(&self) -> impl Iterator<Item=(&T, &C)> + '_ {
        self.counts.iter()
    }

    fn is_empty(&self) -> bool {
        self.counts.is_empty() || self.counts.values().all(|v| v == &C::default())
    }
}
