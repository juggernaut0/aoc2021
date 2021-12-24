use std::collections::HashMap;
use std::str::FromStr;
use crate::util::parse_lines;

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
    let instructions: Vec<Instruction> = parse_lines(input).collect();
    // the code is in independent chunks of 18 instructions
    // the only var to be carried over is z
    // each chunk begins with inp w
    // x is always initialized with previous chunk's z mod 26
    // y is always reset to 0

    let mut zs = HashMap::new(); // z value to largest number that produces that z
    let mut chunks = instructions.chunks(18);
    let first = chunks.next().unwrap();
    for w in 1..10 {
        zs.insert(Apu::new().simulate(first, w, 0), w);
    }
    for chunk in chunks {
        log::debug!("{:?}", zs);
        log::info!("{}", zs.len());
        /*{
            let mut dummy = String::new();
            std::io::stdin().read_line(&mut dummy);
        }*/
        let mut new_zs = HashMap::new();
        for (z, n) in &zs {
            for w in 1..10 {
                let new_n = n*10 + w;
                let new_z = Apu::new().simulate(chunk, w, *z);
                if let Some(old_n) = new_zs.get(&new_z).copied() {
                    if (part_1 && new_n > old_n) || (!part_1 && new_n < old_n) {
                        new_zs.insert(new_z, new_n);
                    }
                } else {
                    new_zs.insert(new_z, new_n);
                }
            }
        }
        zs = new_zs;
    }

    format!("{:?}", zs.get(&0))
}

#[derive(Copy, Clone, Debug)]
enum Arg {
    Var(char),
    Value(i64),
}
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Inp(char),
    Add(char, Arg),
    Mul(char, Arg),
    Div(char, Arg),
    Mod(char, Arg),
    Eql(char, Arg),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let instr = parts.next().unwrap();
        let dest = parts.next().unwrap().chars().next().unwrap();
        let arg_str = parts.next().map(|it| {
            if it.chars().any(|c| c.is_ascii_digit()) {
                let val = it.parse().unwrap();
                Arg::Value(val)
            } else {
                Arg::Var(it.chars().next().unwrap())
            }
        });
        Ok(match instr {
            "inp" => Instruction::Inp(dest),
            "add" => Instruction::Add(dest, arg_str.unwrap()),
            "mul" => Instruction::Mul(dest, arg_str.unwrap()),
            "div" => Instruction::Div(dest, arg_str.unwrap()),
            "mod" => Instruction::Mod(dest, arg_str.unwrap()),
            "eql" => Instruction::Eql(dest, arg_str.unwrap()),
            _ => panic!("unknown instr {}", instr),
        })
    }
}

#[derive(Default)]
struct Apu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Apu {
    fn new() -> Apu {
        Apu::default()
    }

    fn get(&self, c: char) -> i64 {
        match c {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!(),
        }
    }

    fn set(&mut self, c: char, value: i64) {
        match c {
            'w' => self.w = value,
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            _ => panic!(),
        }
    }

    fn get_arg(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Var(c) => self.get(c),
            Arg::Value(v) => v,
        }
    }

    // returns value of z var
    fn simulate(&mut self, instructions: &[Instruction], input: i64, z: i64) -> i64 {
        self.z = z;
        for inst in instructions {
            match *inst {
                Instruction::Inp(_) => self.w = input,
                Instruction::Add(d, a) => self.set(d, self.get(d) + self.get_arg(a)),
                Instruction::Mul(d, a) => self.set(d, self.get(d) * self.get_arg(a)),
                Instruction::Div(d, a) => self.set(d, self.get(d) / self.get_arg(a)),
                Instruction::Mod(d, a) => self.set(d, self.get(d) % self.get_arg(a)),
                Instruction::Eql(d, a) => self.set(d, if self.get(d) == self.get_arg(a) { 1 } else { 0 }),
            }
        }
        self.z
    }
}

/*
inp w     w = 1
mul x 0   x = 0
add x z   x = 0
mod x 26  x = 0
div z 1   z = 0
add x 13  x = 13
eql x w   x = 0
eql x 0   x = 1
mul y 0   y = 0
add y 25  y = 25
mul y x   y = 25
add y 1   y = 26
mul z y   z = 0
mul y 0   y = 0
add y w   y = 1
add y 0   y = 1
mul y x   y = 1
add z y   z = 1

w = inp
x = (z % 26 + 13) != w
z = z * (25 * x + 1) + w * x
return z
 */