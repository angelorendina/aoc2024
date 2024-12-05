use std::{collections::HashSet, io::Read};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day05/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day05/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut lines = buffer.lines();

    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('|'))
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect::<HashSet<_>>();

    let mut total = 0;

    'line: for line in lines {
        let values = line
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let len = values.len();
        for (i, &x) in values.iter().enumerate().take(len - 1) {
            for &y in values.iter().skip(i + 1) {
                if rules.contains(&(y, x)) {
                    // rule broken
                    continue 'line;
                }
            }
        }

        total += values[len / 2];
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day05/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day05/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut lines = buffer.lines();

    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.split_once('|'))
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect::<HashSet<_>>();

    let mut total = 0;

    'line: for line in lines {
        let mut values = line
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let len = values.len();
        for i in 0..(len - 1) {
            let x = values[i];
            for j in (i + 1)..len {
                let y = values[j];
                if rules.contains(&(y, x)) {
                    // rule broken
                    values.sort_unstable_by(|a, b| {
                        if rules.contains(&(*a, *b)) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });

                    total += values[len / 2];

                    continue 'line;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 143);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 123);
    }
}
