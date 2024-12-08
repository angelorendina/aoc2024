use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day08/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day08/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut map = HashMap::new();
    let mut position_by_frequency: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (row, line) in buffer.lines().enumerate() {
        for (column, cell) in line.chars().enumerate() {
            let coords = (row as isize, column as isize);
            map.insert(coords, cell);
            if cell != '.' {
                position_by_frequency.entry(cell).or_default().push(coords);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for antennas in position_by_frequency.values() {
        for &start in antennas {
            for &end in antennas {
                if start == end {
                    continue;
                }
                let delta_row = end.0 - start.0;
                let delta_column = end.1 - start.1;
                let antinode = (end.0 + delta_row, end.1 + delta_column);
                if map.contains_key(&antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day08/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day08/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut map = HashMap::new();
    let mut position_by_frequency: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (row, line) in buffer.lines().enumerate() {
        for (column, cell) in line.chars().enumerate() {
            let coords = (row as isize, column as isize);
            map.insert(coords, cell);
            if cell != '.' {
                position_by_frequency.entry(cell).or_default().push(coords);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for antennas in position_by_frequency.values() {
        for &start in antennas {
            for &end in antennas {
                if start == end {
                    antinodes.insert(start);
                    continue;
                }
                let delta_row = end.0 - start.0;
                let delta_column = end.1 - start.1;
                let mut antinode = (end.0 + delta_row, end.1 + delta_column);
                loop {
                    if map.contains_key(&antinode) {
                        antinodes.insert(antinode);
                        antinode.0 += delta_row;
                        antinode.1 += delta_column;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 14);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 34);
    }
}
