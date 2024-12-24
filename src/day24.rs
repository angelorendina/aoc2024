use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day24/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day24/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut wires = buffer
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let value = value.parse::<u8>().unwrap();
            (name, value)
        })
        .collect::<HashMap<_, _>>();

    let mut ops = buffer
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let (line, target) = line.split_once(" -> ").unwrap();
            let (a, line) = line.split_once(' ').unwrap();
            let (op, b) = line.split_once(' ').unwrap();
            (a, b, op, target)
        })
        .collect::<VecDeque<_>>();

    while let Some((a, b, op, target)) = ops.pop_front() {
        let (a, b) = match (wires.get(a), wires.get(b)) {
            (Some(a), Some(b)) => (*a, *b),
            _ => {
                ops.push_back((a, b, op, target));
                continue;
            }
        };

        match op {
            "AND" => {
                wires.insert(target, a & b);
            }
            "OR" => {
                wires.insert(target, a | b);
            }
            "XOR" => {
                wires.insert(target, a ^ b);
            }
            _ => unreachable!(),
        }
    }

    let z = wires
        .into_iter()
        .filter_map(|(name, value)| {
            name.strip_prefix('z')
                .map(|index| (index.parse::<usize>().unwrap(), value))
        })
        .collect::<BTreeMap<_, _>>();

    z.into_iter()
        .fold(0, |result, (index, bit)| result | (bit as u64) << index)
}

pub fn star_two() -> String {
    const INPUT: &str = "data/day24/input.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    type Target<'a> = &'a str;
    type Register<'a> = &'a str;
    type Operation<'a> = &'a str;

    let mut ops = HashMap::<Target, (Register, Register, Operation)>::new();

    for line in buffer.lines().skip_while(|line| !line.is_empty()).skip(1) {
        let (line, target) = line.split_once(" -> ").unwrap();
        let (a, line) = line.split_once(' ').unwrap();
        let (op, b) = line.split_once(' ').unwrap();
        ops.insert(target, (a, b, op));
    }

    // delightfully hardcoding the right swap so that the the iteration below works
    // (obviously done iteratively when coding, each failure at a time)
    let tmp = ops.remove("z39").unwrap();
    let tmp = ops.insert("ckb", tmp).unwrap();
    ops.insert("z39", tmp);

    let tmp = ops.remove("z20").unwrap();
    let tmp = ops.insert("tqq", tmp).unwrap();
    ops.insert("z20", tmp);

    let tmp = ops.remove("nbd").unwrap();
    let tmp = ops.insert("kbs", tmp).unwrap();
    ops.insert("nbd", tmp);

    let tmp = ops.remove("z06").unwrap();
    let tmp = ops.insert("ksv", tmp).unwrap();
    ops.insert("z06", tmp);

    // parse the full-adder structure starting from the end
    let mut output_carry = "z45";
    for n in (1..=44).rev() {
        let output_sum = format!("z{n:02}");
        let input_a = format!("x{n:02}");
        let input_b: String = format!("y{n:02}");
        let (&direct_sum, _) = ops
            .iter()
            .find(|(_, &(a, b, op))| {
                op == "XOR" && ((a, b) == (&input_a, &input_b) || (b, a) == (&input_a, &input_b))
            })
            .unwrap();
        let (&direct_carry, _) = ops
            .iter()
            .find(|(_, &(a, b, op))| {
                op == "AND" && ((a, b) == (&input_a, &input_b) || (b, a) == (&input_a, &input_b))
            })
            .unwrap();
        let intermediate_carry = ops
            .iter()
            .find_map(|(target, &(a, b, op))| {
                if target == &output_carry && op == "OR" {
                    if a == direct_carry {
                        Some(b)
                    } else if b == direct_carry {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();
        let input_carry = ops
            .iter()
            .find_map(|(target, &(a, b, op))| {
                if target == &intermediate_carry && op == "AND" {
                    if a == direct_sum {
                        Some(b)
                    } else if b == direct_sum {
                        Some(a)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();

        ops.iter()
            .find(|(&target, &(a, b, op))| {
                if target == output_sum && op == "XOR" {
                    (a, b) == (direct_sum, input_carry) || (b, a) == (direct_sum, input_carry)
                } else {
                    false
                }
            })
            .unwrap();

        output_carry = input_carry;
    }

    let mut w = ["z39", "ckb", "z20", "tqq", "nbd", "kbs", "z06", "ksv"];
    w.sort();
    w.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 2024);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), "ckb,kbs,ksv,nbd,tqq,z06,z20,z39");
    }
}
