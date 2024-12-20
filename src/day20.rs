use std::{
    collections::{BTreeMap, HashMap},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day20/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day20/test.txt";

    #[cfg(not(test))]
    const FAST_THRESHOLD: isize = 100;
    #[cfg(test)]
    const FAST_THRESHOLD: isize = 20;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    get_best_shortcuts(&buffer, 2, FAST_THRESHOLD)
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day20/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day20/test.txt";

    #[cfg(not(test))]
    const FAST_THRESHOLD: isize = 100;
    #[cfg(test)]
    const FAST_THRESHOLD: isize = 50;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    get_best_shortcuts(&buffer, 20, FAST_THRESHOLD)
}

enum Tile {
    Wall,
    Track(u64),
}

struct Race {
    at: (isize, isize),
    steps: u64,
}

fn get_best_shortcuts(input: &str, max_cheat: isize, threshold: isize) -> u64 {
    let start = std::cell::OnceCell::new();
    let end = std::cell::OnceCell::new();

    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let start = &start;
            let end = &end;
            line.chars().enumerate().map(move |(column, tile)| {
                let coords = (row as isize, column as isize);
                match tile {
                    '#' => (coords, Tile::Wall),
                    '.' => (coords, Tile::Track(0)),
                    'S' => {
                        start.get_or_init(|| coords);
                        (coords, Tile::Track(0))
                    }
                    'E' => {
                        end.get_or_init(|| coords);
                        (coords, Tile::Track(0))
                    }
                    _ => unreachable!(),
                }
            })
        })
        .collect::<HashMap<_, _>>();

    let start = start.into_inner().unwrap();
    let end = end.into_inner().unwrap();

    let mut tile_by_track = BTreeMap::from([(0, start)]);

    let mut legit_racer = Race {
        at: start,
        steps: 0,
    };
    while legit_racer.at != end {
        for neigh in [
            (legit_racer.at.0 + 1, legit_racer.at.1),
            (legit_racer.at.0 - 1, legit_racer.at.1),
            (legit_racer.at.0, legit_racer.at.1 + 1),
            (legit_racer.at.0, legit_racer.at.1 - 1),
        ] {
            if let Some(Tile::Track(t)) = map.get_mut(&neigh) {
                if *t == 0 && neigh != start {
                    legit_racer.steps += 1;
                    legit_racer.at = neigh;
                    *t = legit_racer.steps;
                    tile_by_track.insert(legit_racer.steps, neigh);
                    break;
                }
            }
        }
    }

    let mut total = 0;

    // # # 3 4 5 6 #
    // S 1 2 # # 7 E
    // legit_racer.steps = 8

    // # # . . . . #
    // S 1 2 # # 7 E
    // t0 = 2, leap_from = (2, 0)
    // t1 = 7, leap_to = (5, 0)
    // t_cheat = |dx| + |dy|

    // saved_steps = (7-2) - 3
    // saved_steps = (t1 - t0) - t_cheat

    for t0 in 0..=legit_racer.steps {
        let leap_from = tile_by_track[&t0];
        for (&t1, &leap_to) in tile_by_track.range(t0..) {
            let cheat_steps =
                isize::abs(leap_from.0 - leap_to.0) + isize::abs(leap_from.1 - leap_to.1);

            if cheat_steps <= max_cheat {
                let saved_steps = (t1 - t0) as isize - cheat_steps;
                if saved_steps >= threshold {
                    total += 1;
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
        assert_eq!(star_one(), 5);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 285);
    }
}
