use std::{collections::VecDeque, io::Read};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day07/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day07/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    'line: for line in buffer.lines() {
        let (result, line) = line.split_once(':').unwrap();
        let result = result.parse::<u64>().unwrap();
        let line = line
            .trim()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .rev()
            .collect::<Vec<_>>();

        let mut exploration = VecDeque::from([(result, line.as_slice())]);

        while let Some((accumulated, numbers)) = exploration.pop_front() {
            let (&next_number, numbers) = numbers.split_first().unwrap();

            // check addition
            if let Some(accumulated) = accumulated.checked_sub(next_number) {
                match (accumulated, numbers.is_empty()) {
                    (0, true) => {
                        total += result;
                        continue 'line;
                    }
                    (0, false) => {}
                    (_accumulated, true) => {}
                    (accumulated, false) => {
                        exploration.push_front((accumulated, numbers));
                    }
                }
            }

            // check multiplication
            if accumulated % next_number == 0 {
                match (accumulated, numbers.is_empty()) {
                    (accumulated, true) if accumulated == next_number => {
                        total += result;
                        continue 'line;
                    }
                    (_, true) => {}
                    (accumulated, false) => {
                        exploration.push_front((accumulated / next_number, numbers));
                    }
                }
            }
        }
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day07/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day07/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    'line: for line in buffer.lines() {
        let (result, line) = line.split_once(':').unwrap();
        let result = result.parse::<u64>().unwrap();
        let line = line
            .trim()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .rev()
            .collect::<Vec<_>>();

        let mut exploration = VecDeque::from([(result, line.as_slice())]);

        while let Some((accumulated, numbers)) = exploration.pop_front() {
            let (&next_number, numbers) = numbers.split_first().unwrap();

            // check addition
            if let Some(accumulated) = accumulated.checked_sub(next_number) {
                match (accumulated, numbers.is_empty()) {
                    (0, true) => {
                        total += result;
                        continue 'line;
                    }
                    (0, false) => {}
                    (_accumulated, true) => {}
                    (accumulated, false) => {
                        exploration.push_front((accumulated, numbers));
                    }
                }
            }

            // check multiplication
            if accumulated % next_number == 0 {
                match (accumulated, numbers.is_empty()) {
                    (accumulated, true) if accumulated == next_number => {
                        total += result;
                        continue 'line;
                    }
                    (_, true) => {}
                    (accumulated, false) => {
                        exploration.push_front((accumulated / next_number, numbers));
                    }
                }
            }

            // check adjoining
            if !numbers.is_empty() {
                if let Some(accumulated) = accumulated.checked_sub(next_number) {
                    let offset = {
                        let mut offset = 10;
                        while offset <= next_number {
                            offset *= 10;
                        }
                        offset
                    };
                    if accumulated % offset == 0 {
                        let accumulated = accumulated / offset;
                        if accumulated > 0 {
                            exploration.push_front((accumulated, numbers));
                        }
                    }
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
        assert_eq!(star_one(), 3749);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 11387);
    }
}
