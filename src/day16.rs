pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       let packet = new_parser(&input).parse_packet();

       format!("{}", packet.sum_version())
   }

   fn solve_2(&self, input: String) -> String {
       let packet = new_parser(&input).parse_packet();

       format!("{}", packet.eval())
   }
}

fn parse_input(input: &str) -> Vec<bool> {
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
        .collect()
}

struct Parser<T> {
    stream: T,
    current: usize,
}

fn new_parser(input: &str) -> Parser<impl Iterator<Item=bool>> {
    Parser {
        stream: parse_input(input).into_iter(),
        current: 0,
    }
}

impl<T: Iterator<Item=bool>> Parser<T> {
    fn next(&mut self) -> bool {
        self.current += 1;
        self.stream.next().unwrap()
    }

    fn parse_packet(&mut self) -> Packet {
        let version = self.parse_n(3) as u8;
        let type_id = self.parse_n(3) as u8;
        let packet_type = if type_id == 4 {
            PacketType::Literal(self.parse_literal())
        } else {
            PacketType::Operator { children: self.parse_operator() }
        };
        Packet { version, type_id, packet_type }
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

    fn parse_operator(&mut self) -> Vec<Packet> {
        let length_type = self.next();
        let mut children = Vec::new();
        if length_type {
            let n = self.parse_n(11);
            for _ in 0..n {
                children.push(self.parse_packet());
            }
        } else {
            let n = self.parse_n(15) as usize;
            let start = self.current;
            while self.current - start < n {
                children.push(self.parse_packet());
            }
        }
        children
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType,
}

impl Packet {
    fn sum_version(&self) -> u64 {
        self.version as u64 + match &self.packet_type {
            PacketType::Literal(_) => 0,
            PacketType::Operator { children } => {
                children.iter().map(|it| it.sum_version()).sum()
            }
        }
    }

    fn eval(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(n) => *n,
            PacketType::Operator { children } => {
                match self.type_id {
                    0 => children.iter().map(|it| it.eval()).sum(),
                    1 => children.iter().map(|it| it.eval()).product(),
                    2 => children.iter().map(|it| it.eval()).min().unwrap(),
                    3 => children.iter().map(|it| it.eval()).max().unwrap(),
                    5 => if children[0].eval() > children[1].eval() { 1 } else { 0 },
                    6 => if children[0].eval() < children[1].eval() { 1 } else { 0 },
                    7 => if children[0].eval() == children[1].eval() { 1 } else { 0 },
                    unknown => panic!("unknown type id {}", unknown),
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum PacketType {
    Literal(u64),
    Operator { children: Vec<Packet> }
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
        assert_eq!(exp, parse_input("A41"));
    }

    #[test]
    fn test_parse_n() {
        let mut parser = new_parser("A41");
        let n = parser.parse_n(6);
        assert_eq!(41, n);
    }

    #[test]
    fn test_parse_literal() {
        let mut parser = new_parser("D2FE28");
        assert_eq!(Packet { version: 6, type_id: 4, packet_type: PacketType::Literal(2021) }, parser.parse_packet())
    }

    #[test]
    fn test_operator_0() {
        let mut parser = new_parser("38006F45291200");
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
        assert_eq!(exp, parser.parse_packet())
    }
}