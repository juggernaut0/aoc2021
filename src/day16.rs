pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let result = Parser::of_str(&input, VersionVisitor::new()).parse().result();

       format!("{}", result)
   }

   fn solve_2(&self, input: String) -> String {
       let result = Parser::of_str(&input, EvalVisitor::new()).parse().result();

       format!("{}", result)
   }
}

fn parse_input(input: &str) -> impl Iterator<Item=bool> + '_ {
    input.trim().chars()
        .flat_map(|c| {
            let mut buf = [0; 4];
            let s = c.encode_utf8(&mut buf);
            let n = u8::from_str_radix(s, 16).unwrap();
            [
                n & 0b1000 == 0b1000,
                n & 0b100 == 0b100,
                n & 0b10 == 0b10,
                n & 0b1 == 0b1,
            ]
        })
}

struct Parser<T, V: PacketVisitor> {
    stream: T,
    current: usize,
    visitor: V,
}

impl<V: PacketVisitor> Parser<(), V> {
    fn of_str(input: &str, visitor: V) -> Parser<impl Iterator<Item=bool> + '_, V> {
        Parser::new(parse_input(input), visitor)
    }

    fn new<B: IntoIterator<Item=bool>>(b: B, visitor: V) -> Parser<B::IntoIter, V> {
        Parser {
            stream: b.into_iter(),
            current: 0,
            visitor
        }
    }
}

impl<T: Iterator<Item=bool>, V: PacketVisitor> Parser<T, V> {
    fn parse(mut self) -> V {
        self.parse_packet();
        self.visitor
    }

    fn next(&mut self) -> bool {
        self.current += 1;
        self.stream.next().unwrap()
    }

    fn parse_packet(&mut self) {
        let version = self.parse_n(3) as u8;
        let type_id = self.parse_n(3) as u8;
        self.visitor.begin(version, type_id);
        if type_id == 4 {
            let v = self.parse_literal();
            self.visitor.end_literal(v);
        } else {
            self.parse_operator();
            self.visitor.end_operator();
        }
    }

    fn parse_n(&mut self, bits: u8) -> u32 {
        let mut res = 0;
        for _ in 0..bits {
            res <<= 1;
            if self.next() {
                res += 1;
            }
        }
        res
    }

    fn parse_literal(&mut self) -> u64 {
        let mut res = 0;
        loop {
            let cont = self.next();
            let n = self.parse_n(4);
            res <<= 4;
            res |= n as u64;
            if !cont {
                break
            }
        }
        res
    }

    fn parse_operator(&mut self) {
        let length_type = self.next();
        if length_type {
            let n = self.parse_n(11);
            for _ in 0..n {
                self.parse_packet();
            }
        } else {
            let n = self.parse_n(15) as usize;
            let start = self.current;
            while self.current - start < n {
                self.parse_packet();
            }
        }
    }
}

#[allow(unused_variables)]
trait PacketVisitor {
    fn begin(&mut self, version: u8, type_id: u8) {}
    fn end_literal(&mut self, value: u64) {}
    fn end_operator(&mut self) {}
}

struct VersionVisitor {
    total: u32,
}

impl VersionVisitor {
    fn new() -> VersionVisitor {
        VersionVisitor {
            total: 0,
        }
    }

    fn result(self) -> u32 {
        self.total
    }
}

impl PacketVisitor for VersionVisitor {
    fn begin(&mut self, version: u8, _type_id: u8) {
        self.total += version as u32;
    }
}

struct EvalVisitor {
    stack: Vec<EvalFrame>,
}

impl EvalVisitor {
    fn new() -> EvalVisitor {
        EvalVisitor {
            stack: vec![EvalFrame { type_id: 0, values: Vec::new() }],
        }
    }

    fn result(self) -> u64 {
        self.stack[0].values[0]
    }
}

impl PacketVisitor for EvalVisitor {
    fn begin(&mut self, _version: u8, type_id: u8) {
        self.stack.push(EvalFrame { type_id, values: Vec::new() });
    }

    fn end_literal(&mut self, value: u64) {
        self.stack.pop().unwrap();
        self.stack.last_mut().unwrap().values.push(value);
    }

    fn end_operator(&mut self) {
        let EvalFrame { type_id, values } = self.stack.pop().unwrap();
        let value = match type_id {
            0 => values.into_iter().sum(),
            1 => values.into_iter().product(),
            2 => values.into_iter().min().unwrap(),
            3 => values.into_iter().max().unwrap(),
            5 => if values[0] > values[1] { 1 } else { 0 },
            6 => if values[0] < values[1] { 1 } else { 0 },
            7 => if values[0] == values[1] { 1 } else { 0 },
            unknown => panic!("unknown type id {}", unknown),
        };
        self.stack.last_mut().unwrap().values.push(value);
    }
}

struct EvalFrame {
    type_id: u8,
    values: Vec<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        crate::init_test_logging();
        let exp = vec![
            true, false, true, false,
            false, true, false, false,
            false, false, false, true,
        ];
        assert_eq!(exp, parse_input("A41").collect::<Vec<_>>());
    }

    #[test]
    fn test_parse_n() {
        let n = Parser::of_str("A41").parse_n(6);
        assert_eq!(41, n);
    }

    #[test]
    fn test_parse_literal() {
        let packet = Parser::of_str("D2FE28").parse_packet();
        assert_eq!(Packet { version: 6, type_id: 4, packet_type: PacketType::Literal(2021) }, packet)
    }

    #[test]
    fn test_operator_0() {
        let packet = Parser::of_str("38006F45291200").parse_packet();
        let exp = Packet {
            version: 1,
            type_id: 6,
            packet_type: PacketType::Operator {
                children: vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        packet_type: PacketType::Literal(10)
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        packet_type: PacketType::Literal(20)
                    },
                ]
            }
        };
        assert_eq!(exp, packet)
    }
}