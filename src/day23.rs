use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day23/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day23/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut pcs = HashSet::<&str>::new();
    let mut pairs = HashSet::<(&str, &str)>::new();

    let mut total = 0;

    for line in buffer.lines() {
        let (a, b) = line.split_once('-').unwrap();
        pcs.insert(a);
        pcs.insert(b);
        pairs.insert((a, b));
        pairs.insert((b, a));

        for c in &pcs {
            if (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                && pairs.contains(&(a, c))
                && pairs.contains(&(b, c))
            {
                total += 1;
            }
        }
    }

    total
}

pub fn star_two() -> String {
    #[cfg(not(test))]
    const INPUT: &str = "data/day23/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day23/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut edges = HashMap::<&str, HashSet<&str>>::new();

    for line in buffer.lines() {
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a).or_default().insert(b);
        edges.entry(b).or_default().insert(a);
    }

    let mut largest_lobby = HashSet::<&str>::new();

    for (&a, neighs) in &edges {
        for &b in neighs {
            let mut lobby = HashSet::from([a, b]);
            for (&c, e) in &edges {
                if !lobby.contains(c) {
                    let mut adjoin = true;
                    for &x in &lobby {
                        if !e.contains(x) {
                            adjoin = false;
                            break;
                        }
                    }
                    if adjoin {
                        lobby.insert(c);
                    }
                }
            }
            if lobby.len() > largest_lobby.len() {
                largest_lobby = lobby;
            }
        }
    }

    let mut largest_lobby = largest_lobby.into_iter().collect::<Vec<_>>();
    largest_lobby.sort();
    largest_lobby.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 7);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), "co,de,ka,ta");
    }
}
