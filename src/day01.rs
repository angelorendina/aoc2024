use std::collections::HashMap;
use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day01/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day01/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let (mut list_one, mut list_two): (Vec<_>, Vec<_>) = buffer
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(a, b)| {
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .unzip();

    list_one.sort();
    list_two.sort();

    list_one
        .into_iter()
        .zip(list_two)
        .map(|(a, b)| u64::abs_diff(a, b))
        .sum()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day01/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day01/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let (keys, mut values): (Vec<u64>, HashMap<u64, u64>) = buffer
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(a, b)| {
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut keys, mut values), (key, value)| {
                keys.push(key);
                *values.entry(value).or_default() += 1;

                (keys, values)
            },
        );

    keys.into_iter()
        .map(|key| key * *values.entry(key).or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 11);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 31);
    }
}
