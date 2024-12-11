use std::{collections::HashMap, io::Read};

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day11/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day11/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let line = buffer
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let mut memo = HashMap::new();

    line.map(|x| stones_memo(x, 25, &mut memo)).sum()
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day11/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day11/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let line = buffer
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let mut memo = HashMap::new();

    line.map(|x| stones_memo(x, 75, &mut memo)).sum()
}

fn stones_memo(x: u64, depth: u8, memo: &mut HashMap<(u64, u8), usize>) -> usize {
    if depth == 0 {
        1
    } else {
        let [y, z] = stone(x);
        let y = match y {
            None => 0,
            Some(y) => {
                let ys = memo.get(&(y, depth - 1)).copied();
                match ys {
                    None => {
                        let ys = stones_memo(y, depth - 1, memo);
                        memo.insert((y, depth - 1), ys);
                        ys
                    }
                    Some(ys) => ys,
                }
            }
        };
        let z = match z {
            None => 0,
            Some(z) => {
                let zs = memo.get(&(z, depth - 1)).copied();
                match zs {
                    None => {
                        let zs = stones_memo(z, depth - 1, memo);
                        memo.insert((z, depth - 1), zs);
                        zs
                    }
                    Some(zs) => zs,
                }
            }
        };

        y + z
    }
}

fn stone(x: u64) -> [Option<u64>; 2] {
    match x {
        0 => [Some(1), None],
        x => {
            let digits = x.ilog10() + 1;
            if digits % 2 == 0 {
                let p = 10_u64.pow(digits / 2);

                let top_half = x / p;
                let bottom_half = x % p;

                [Some(top_half), Some(bottom_half)]
            } else {
                [Some(x * 2024), None]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 55312);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 65601038650482);
    }
}
