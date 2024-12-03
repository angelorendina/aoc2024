use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day03/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day03/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    for start in 0..buffer.len() {
        let end = buffer.len().min(start + 12);
        let window = &buffer[start..end];
        let Some((window, _)) = window.split_once(')') else {
            continue;
        };
        let Some(("mul", window)) = window.split_once('(') else {
            continue;
        };
        let Some((a, b)) = window.split_once(',') else {
            continue;
        };
        if a.is_empty() || b.is_empty() || a.len() > 3 || b.len() > 3 {
            continue;
        }
        if !a.chars().all(char::is_numeric) || !b.chars().all(char::is_numeric) {
            continue;
        }
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        total += a * b;
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day03/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day03/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;
    let mut enabled = true;

    for start in 0..buffer.len() {
        let end = buffer.len().min(start + 12);
        let window = &buffer[start..end];

        if window.starts_with("do()") {
            enabled = true;
            continue;
        }
        if window.starts_with("don't()") {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let Some((window, _)) = window.split_once(')') else {
            continue;
        };
        let Some(("mul", window)) = window.split_once('(') else {
            continue;
        };
        let Some((a, b)) = window.split_once(',') else {
            continue;
        };
        if a.is_empty() || b.is_empty() || a.len() > 3 || b.len() > 3 {
            continue;
        }
        if !a.chars().all(char::is_numeric) || !b.chars().all(char::is_numeric) {
            continue;
        }
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        total += a * b;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 161);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 48);
    }
}
