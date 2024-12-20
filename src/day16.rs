use std::{
    collections::{HashMap, HashSet, VecDeque},
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

    let cost_from_start = calculate_cost_from(&map, (start.0, start.1, Direction::East));

    best_cost_to(&cost_from_start, end)
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

    let cost_from_start = calculate_cost_from(&map, (start.0, start.1, Direction::East));
    let cost_from_end = calculate_cost_to(&map, end);

    let best_cost = best_cost_to(&cost_from_start, end);

    let mut total = 0;

    for (row, column) in map {
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let Some(&x) = cost_from_start.get(&(row, column, direction)) else {
                continue;
            };
            let Some(&y) = cost_from_end.get(&(row, column, direction)) else {
                continue;
            };
            if x + y == best_cost {
                total += 1;
                // do not count same tile multiple times
                break;
            }
        }
    }

    total
}

#[allow(clippy::type_complexity)]
fn parse_map(input: &str) -> (HashSet<(isize, isize)>, (isize, isize), (isize, isize)) {
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
                    '.' => Some(tile),
                    'S' => {
                        start.set(tile).unwrap();
                        Some(tile)
                    }
                    'E' => {
                        end.set(tile).unwrap();
                        Some(tile)
                    }
                    _ => None,
                }
            })
        })
        .collect::<HashSet<_>>();

    (map, start.into_inner().unwrap(), end.into_inner().unwrap())
}

fn calculate_cost_from(
    map: &HashSet<(isize, isize)>,
    init: (isize, isize, Direction),
) -> HashMap<(isize, isize, Direction), u64> {
    let mut costs = map
        .iter()
        .flat_map(|&(row, column)| {
            [
                ((row, column, Direction::East), u64::MAX),
                ((row, column, Direction::North), u64::MAX),
                ((row, column, Direction::South), u64::MAX),
                ((row, column, Direction::West), u64::MAX),
            ]
        })
        .collect::<HashMap<_, _>>();

    let mut exploration = VecDeque::from([
        ((init.0, init.1, init.2.rotated_clockwise_times(0)), 0u64),
        ((init.0, init.1, init.2.rotated_clockwise_times(1)), 1000),
        ((init.0, init.1, init.2.rotated_clockwise_times(2)), 1000),
        ((init.0, init.1, init.2.rotated_clockwise_times(3)), 2000),
    ]);

    while let Some(((row, column, direction), cost)) = exploration.pop_front() {
        let Some(previous_best_cost) = costs.get_mut(&(row, column, direction)) else {
            continue;
        };
        if cost >= *previous_best_cost {
            continue;
        }
        *previous_best_cost = cost;

        let front_tile = direction.projected_from((row, column));
        if map.contains(&front_tile) {
            exploration.push_back(((front_tile.0, front_tile.1, direction), cost + 1));
        }

        exploration.push_back(((row, column, direction.rotated_clockwise()), cost + 1000));
        exploration.push_back((
            (row, column, direction.rotated_clockwise_times(3)),
            cost + 1000,
        ));
    }

    costs
}

fn best_cost_to(costs: &HashMap<(isize, isize, Direction), u64>, target: (isize, isize)) -> u64 {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .filter_map(|direction| costs.get(&(target.0, target.1, direction)))
    .min()
    .copied()
    .unwrap()
}

fn calculate_cost_to(
    map: &HashSet<(isize, isize)>,
    target: (isize, isize),
) -> HashMap<(isize, isize, Direction), u64> {
    let mut costs = map
        .iter()
        .flat_map(|&(row, column)| {
            [
                ((row, column, Direction::East), u64::MAX),
                ((row, column, Direction::North), u64::MAX),
                ((row, column, Direction::South), u64::MAX),
                ((row, column, Direction::West), u64::MAX),
            ]
        })
        .collect::<HashMap<_, _>>();

    let mut exploration = VecDeque::from([
        ((target.0, target.1, Direction::East), 0u64),
        ((target.0, target.1, Direction::North), 0),
        ((target.0, target.1, Direction::South), 0),
        ((target.0, target.1, Direction::West), 0),
    ]);

    while let Some(((row, column, direction), cost)) = exploration.pop_front() {
        let Some(previous_best_cost) = costs.get_mut(&(row, column, direction)) else {
            continue;
        };
        if cost >= *previous_best_cost {
            continue;
        }
        *previous_best_cost = cost;

        let back_tile = direction
            .rotated_clockwise_times(2)
            .projected_from((row, column));
        if map.contains(&back_tile) {
            exploration.push_back(((back_tile.0, back_tile.1, direction), cost + 1));
        }

        exploration.push_back(((row, column, direction.rotated_clockwise()), cost + 1000));
        exploration.push_back((
            (row, column, direction.rotated_clockwise_times(3)),
            cost + 1000,
        ));
    }

    costs
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
