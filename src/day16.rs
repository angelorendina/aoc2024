use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day16/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day16/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let (map, start, end) = parse_map(&buffer);

    calculate_best_path(
        map,
        Exploration {
            position: start,
            direction: Direction::East,
            cost: 0,
        },
        end,
    )
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day16/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day16/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let (map, start, end) = parse_map(&buffer);

    let best_cost = calculate_best_path(
        map.clone(),
        Exploration {
            position: start,
            direction: Direction::East,
            cost: 0,
        },
        end,
    );

    let mut exploration = std::collections::BinaryHeap::from([Exploration {
        direction: Direction::East,
        position: start,
        cost: 0,
    }]);

    let mut best_seats = HashSet::from([start]);

    // flood-fill computing Start -> Midpoint -> End for all possible midpoints
    while let Some(e) = exploration.pop() {
        for i in 0u64..4 {
            let next_direction = e.direction.rotated_clockwise_times(i);
            let next_position = next_direction.projected_from(e.position);
            if best_seats.contains(&next_position) {
                continue;
            }
            if map.contains_key(&next_position) {
                let next_cost = e.cost
                    + match i {
                        0 => 1,
                        1 => 1001,
                        2 => 2001,
                        3 => 1001,
                        _ => unreachable!(),
                    };
                // hopeless, skip this
                if next_cost > best_cost {
                    continue;
                }
                let prospected_cost = calculate_best_path(
                    map.clone(),
                    Exploration {
                        direction: next_direction,
                        position: next_position,
                        cost: next_cost,
                    },
                    end,
                );
                if prospected_cost == best_cost {
                    best_seats.insert(next_position);
                    exploration.push(Exploration {
                        direction: next_direction,
                        position: next_position,
                        cost: next_cost,
                    });
                }
            }
        }
    }

    best_seats.len() as u64
}

#[allow(clippy::type_complexity)]
fn parse_map(input: &str) -> (HashMap<(isize, isize), u64>, (isize, isize), (isize, isize)) {
    let start = std::cell::OnceCell::new();
    let end = std::cell::OnceCell::new();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let start = &start;
            let end = &end;
            line.chars().enumerate().filter_map(move |(column, c)| {
                let tile = (row as isize, column as isize);
                match c {
                    '.' => Some((tile, u64::MAX)),
                    'S' => {
                        start.set(tile).unwrap();
                        Some((tile, u64::MAX))
                    }
                    'E' => {
                        end.set(tile).unwrap();
                        Some((tile, u64::MAX))
                    }
                    _ => None,
                }
            })
        })
        .collect::<HashMap<_, _>>();

    (map, start.into_inner().unwrap(), end.into_inner().unwrap())
}

fn calculate_best_path(
    mut map: HashMap<(isize, isize), u64>,
    start: Exploration,
    end: (isize, isize),
) -> u64 {
    map.insert(start.position, start.cost);

    let mut exploration = std::collections::BinaryHeap::from([start]);

    // flood-fill exploration
    while let Some(e) = exploration.pop() {
        for i in 0u64..4 {
            let next_direction = e.direction.rotated_clockwise_times(i);
            let next_position = next_direction.projected_from(e.position);
            if let Some(cached_cost) = map.get_mut(&next_position) {
                let next_cost = e.cost
                    + match i {
                        0 => 1,
                        1 => 1001,
                        2 => 2001,
                        3 => 1001,
                        _ => unreachable!(),
                    };
                if next_cost < *cached_cost {
                    *cached_cost = next_cost;
                    exploration.push(Exploration {
                        direction: next_direction,
                        position: next_position,
                        cost: next_cost,
                    });
                }
            }
        }
    }

    *map.get(&end).unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotated_clockwise_times(self, times: u64) -> Self {
        match times % 4 {
            0 => self,
            1 => self.rotated_clockwise(),
            2 => self.rotated_clockwise().rotated_clockwise(),
            3 => self
                .rotated_clockwise()
                .rotated_clockwise()
                .rotated_clockwise(),
            _ => unreachable!(),
        }
    }

    fn rotated_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn projected_from(&self, position: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::North => (position.0 - 1, position.1),
            Direction::East => (position.0, position.1 + 1),
            Direction::South => (position.0 + 1, position.1),
            Direction::West => (position.0, position.1 - 1),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Exploration {
    direction: Direction,
    position: (isize, isize),
    cost: u64,
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Exploration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl Ord for Exploration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 7036);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 45);
    }
}
