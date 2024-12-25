use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day25/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day25/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut locks = vec![];
    let mut keys = vec![];

    let mut lines = buffer.lines();

    fn parse_line(line: &str) -> [bool; 5] {
        std::array::from_fn(|i| line.chars().nth(i).unwrap() == '#')
    }

    while let Some(line) = lines.next() {
        let is_lock = line.starts_with('#');
        let rows = [
            parse_line(lines.next().unwrap()),
            parse_line(lines.next().unwrap()),
            parse_line(lines.next().unwrap()),
            parse_line(lines.next().unwrap()),
            parse_line(lines.next().unwrap()),
        ];
        // consume bottom of frame and trailing empty line, if any
        lines.next();
        lines.next();

        let combination =
            std::array::from_fn::<_, 5, _>(|i| rows.iter().filter(|row| row[i]).count());
        if is_lock {
            locks.push(combination);
        } else {
            keys.push(combination);
        }
    }

    let mut total = 0;

    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(&l, &k)| l + k <= 5) {
                total += 1;
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
        assert_eq!(star_one(), 3);
    }
}
