use std::io::Read;

pub fn star_one() -> String {
    #[cfg(not(test))]
    const INPUT: &str = "data/day17/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day17/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut vm = VM::parse(&buffer);

    let mut out = vec![];

    while vm.step(&mut out) {}

    out.into_iter()
        .map(|x| format!("{x}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn star_two() -> u64 {
    let text = [2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 4, 5, 5, 3, 0];

    find_a(0, &text).unwrap()
}

// disgusting by-hand decompilation
fn compiled(a: u64) -> u8 {
    let b = a % 8;
    let b = b ^ 3;
    let c = a >> b;
    let b = b ^ 5;
    ((b ^ c) % 8) as u8
}

fn find_a(a: u64, text: &[u8]) -> Option<u64> {
    let Some((&last, text)) = text.split_last() else {
        return Some(a);
    };

    for i in 0..8 {
        let a = a * 8 + i;
        if compiled(a) == last {
            if let Some(res) = find_a(a, text) {
                return Some(res);
            }
        }
    }

    None
}

struct VM {
    a: u64,
    b: u64,
    c: u64,
    text: Vec<u8>,
    ip: usize,
}

impl VM {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let (_, a) = lines.next().unwrap().split_once("A: ").unwrap();
        let (_, b) = lines.next().unwrap().split_once("B: ").unwrap();
        let (_, c) = lines.next().unwrap().split_once("C: ").unwrap();
        lines.next().unwrap();
        let (_, text) = lines.next().unwrap().split_once(": ").unwrap();

        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
            c: c.parse().unwrap(),
            text: text.split(',').map(|t| t.parse().unwrap()).collect(),
            ip: 0,
        }
    }

    fn step(&mut self, out: &mut Vec<u8>) -> bool {
        let Some(&opcode) = self.text.get(self.ip) else {
            return false;
        };
        let Some(&operand) = self.text.get(self.ip + 1) else {
            return false;
        };

        match opcode {
            0 => {
                let operand = match operand {
                    x @ 0..=3 => x as _,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                };
                self.a >>= operand;
                self.ip += 2;
            }
            1 => {
                self.b ^= operand as u64;
                self.ip += 2;
            }
            2 => {
                let operand = match operand {
                    x @ 0..=3 => x as _,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                };
                self.b = operand % 8;
                self.ip += 2;
            }
            3 => {
                if self.a > 0 {
                    self.ip = operand as usize;
                } else {
                    self.ip += 2;
                }
            }
            4 => {
                self.b ^= self.c;
                self.ip += 2;
            }
            5 => {
                let operand = match operand {
                    x @ 0..=3 => x as _,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                };
                out.push(operand as u8 % 8);
                self.ip += 2;
            }
            6 => {
                let operand = match operand {
                    x @ 0..=3 => x as _,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                };
                self.b = self.a >> operand;
                self.ip += 2;
            }
            7 => {
                let operand = match operand {
                    x @ 0..=3 => x as _,
                    4 => self.a,
                    5 => self.b,
                    6 => self.c,
                    _ => unreachable!(),
                };
                self.c = self.a >> operand;
                self.ip += 2;
            }
            _ => unreachable!(),
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 236539226447469);
    }
}
