use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day10/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day10/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let map = buffer
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                let h = c.to_digit(10).unwrap();
                ((row as isize, column as isize), h as usize)
            })
        })
        .collect::<HashMap<_, _>>();

    let mut total = 0;

    let mut trailheads_targets = HashMap::new();
    let mut explorations = VecDeque::new();

    for (&base, _) in map.iter().filter(|(_, &h)| h == 0) {
        trailheads_targets.insert(base, HashSet::new());
        explorations.push_back(vec![base]);
    }

    while let Some(path) = explorations.pop_front() {
        let height_to = path.len();
        let &(row, column) = path.last().unwrap();
        for tile_to in [
            (row + 1, column),
            (row - 1, column),
            (row, column + 1),
            (row, column - 1),
        ] {
            if let Some(&h) = map.get(&tile_to) {
                if h == height_to {
                    if height_to == 9 {
                        let new_peak = trailheads_targets
                            .get_mut(path.first().unwrap())
                            .unwrap()
                            .insert(tile_to);
                        if new_peak {
                            total += 1;
                        }
                    } else {
                        let mut path = path.clone();
                        path.push(tile_to);
                        explorations.push_back(path);
                    }
                }
            }
        }
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day10/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day10/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let map = buffer
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(column, c)| {
                let h = c.to_digit(10).unwrap();
                ((row as isize, column as isize), h as usize)
            })
        })
        .collect::<HashMap<_, _>>();

    let mut total = 0;

    let mut explorations = map
        .iter()
        .filter(|(_, &h)| h == 0)
        .map(|(&base, _)| vec![base])
        .collect::<VecDeque<_>>();

    while let Some(path) = explorations.pop_front() {
        let height_to = path.len();
        let &(row, column) = path.last().unwrap();
        for tile_to in [
            (row + 1, column),
            (row - 1, column),
            (row, column + 1),
            (row, column - 1),
        ] {
            if let Some(&h) = map.get(&tile_to) {
                if h == height_to {
                    if height_to == 9 {
                        total += 1;
                    } else {
                        let mut path = path.clone();
                        path.push(tile_to);
                        explorations.push_back(path);
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
        assert_eq!(star_one(), 36);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 81);
    }
}
