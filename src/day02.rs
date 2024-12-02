use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day02/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day02/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()))
        .map(|levels| if safety(levels) { 1 } else { 0 })
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day02/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day02/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    buffer
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()))
        .map(|levels| {
            let levels = levels.collect::<Vec<_>>();
            if safety(levels.iter().copied()) || subsafety(levels) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn subsafety(levels: Vec<i64>) -> bool {
    for index_to_skip in 0..levels.len() {
        let sublevel = levels.iter().enumerate().filter_map(|(index_at, n)| {
            if index_at == index_to_skip {
                None
            } else {
                Some(*n)
            }
        });
        if safety(sublevel) {
            return true;
        }
    }

    false
}

fn safety(levels: impl Iterator<Item = i64>) -> bool {
    let mut previous = None;
    let mut anterior = None;

    for a in levels {
        match (anterior, previous) {
            (Some(_), None) => unreachable!(),
            (None, None) => {
                anterior = None;
                previous = Some(a);
            }
            (None, Some(b)) => {
                let delta = i64::checked_sub(b, a).unwrap();
                if delta == 0 || delta.abs() > 3 {
                    return false;
                }
                anterior = Some(b);
                previous = Some(a);
            }
            (Some(c), Some(b)) => {
                let delta = i64::checked_sub(b, a).unwrap();
                if delta == 0 || delta.abs() > 3 {
                    return false;
                }
                let previous_delta = i64::checked_sub(c, b).unwrap();
                if delta.signum() != previous_delta.signum() {
                    return false;
                }
                anterior = Some(b);
                previous = Some(a);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 2);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 4);
    }
}
