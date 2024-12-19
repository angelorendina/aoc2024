use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day19/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day19/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut lines = buffer.lines();

    let patterns = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next();

    let mut bad_tails = HashSet::new();
    let mut total = 0;

    for line in lines {
        if check(line, &patterns, &mut bad_tails) {
            total += 1;
        }
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day19/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day19/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut lines = buffer.lines();

    let patterns = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next();

    let mut cache = HashMap::new();
    let mut total = 0;

    for line in lines {
        total += count(line, &patterns, &mut cache);
    }

    total
}

fn check<'a>(wanted: &'a str, patterns: &'a [&'a str], bad_tails: &mut HashSet<&'a str>) -> bool {
    if bad_tails.contains(wanted) {
        return false;
    }

    if wanted.is_empty() {
        return true;
    }

    for &pattern in patterns {
        if let Some(tail) = wanted.strip_prefix(pattern) {
            if check(tail, patterns, bad_tails) {
                return true;
            }
        }
    }

    bad_tails.insert(wanted);
    false
}

fn count<'a>(wanted: &'a str, patterns: &'a [&'a str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if wanted.is_empty() {
        return 1;
    }

    if let Some(combinations) = cache.get(wanted) {
        return *combinations;
    }

    let mut combinations = 0;
    for &pattern in patterns {
        if let Some(tail) = wanted.strip_prefix(pattern) {
            let subcombos = count(tail, patterns, cache);
            combinations += subcombos;
        }
    }

    cache.insert(wanted, combinations);
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 6);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 16);
    }
}
