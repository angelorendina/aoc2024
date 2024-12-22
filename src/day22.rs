use std::{collections::HashMap, io::Read};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day22/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day22/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    for line in buffer.lines() {
        let mut value = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            value = (value ^ (value * 64)) % 16777216;
            value = (value ^ (value / 32)) % 16777216;
            value = (value ^ (value * 2048)) % 16777216;
        }
        total += value;
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day22/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day22/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut history = HashMap::new();

    let f = |mut x: u64| -> u64 {
        x = (x ^ (x * 64)) % 16777216;
        x = (x ^ (x / 32)) % 16777216;
        x = (x ^ (x * 2048)) % 16777216;
        x
    };

    for (index, line) in buffer.lines().enumerate() {
        let mut window = [0; 5];
        window[0] = line.parse::<u64>().unwrap();
        window[1] = f(window[0]);
        window[2] = f(window[1]);
        window[3] = f(window[2]);
        window[4] = f(window[3]);
        for _ in 4..2000 {
            let changes = [
                (window[1] % 10) as i8 - (window[0] % 10) as i8,
                (window[2] % 10) as i8 - (window[1] % 10) as i8,
                (window[3] % 10) as i8 - (window[2] % 10) as i8,
                (window[4] % 10) as i8 - (window[3] % 10) as i8,
            ];
            history
                .entry(changes)
                .or_insert(HashMap::new())
                .entry(index)
                .or_insert(window[4] % 10);
            window.rotate_left(1);
            window[4] = f(window[3]);
        }
    }

    let mut best_score = 0;
    for (_, scores) in history {
        let score = scores.values().sum::<u64>();
        best_score = best_score.max(score);
    }

    best_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 37327623);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 24);
    }
}
