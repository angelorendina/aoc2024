use std::{collections::HashMap, io::Read};

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day06/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day06/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut simulation = Simulation::parse(&buffer);

    match simulation.run() {
        SimulationOutcome::Exited(tiles) => tiles,
        SimulationOutcome::Looped => unreachable!(),
    }
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day06/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day06/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut simulation = Simulation::parse(&buffer);

    let mut total = 0;

    loop {
        let next_tile = simulation.guard.looking_at();
        if let Some(Tile::Floor) = simulation.map.get(&next_tile) {
            match simulation.visited.get(&next_tile) {
                None => {
                    let mut divergent_simulation = simulation.clone();
                    divergent_simulation.map.insert(next_tile, Tile::Obstacle);
                    if let SimulationOutcome::Looped = divergent_simulation.run() {
                        total += 1;
                    }
                }
                Some(seen_directions)
                    if seen_directions.get_visited(&simulation.guard.direction) =>
                {
                    let mut divergent_simulation = simulation.clone();
                    divergent_simulation.map.insert(next_tile, Tile::Obstacle);
                    if let SimulationOutcome::Looped = divergent_simulation.run() {
                        total += 1;
                    }
                }
                Some(_) => {}
            }
        }
        if simulation.step().is_some() {
            return total;
        }
    }
}

#[derive(Clone)]
struct Simulation {
    guard: Guard,
    map: HashMap<(isize, isize), Tile>,
    visited: HashMap<(isize, isize), VisitedDirections>,
}

enum SimulationOutcome {
    Exited(usize),
    Looped,
}

impl Simulation {
    fn parse(text: &str) -> Self {
        let mut map = HashMap::new();
        let mut guard = None;

        for (row, line) in text.lines().enumerate() {
            for (column, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        map.insert((row as isize, column as isize), Tile::Floor);
                    }
                    '#' => {
                        map.insert((row as isize, column as isize), Tile::Obstacle);
                    }
                    '^' => {
                        map.insert((row as isize, column as isize), Tile::Floor);
                        guard = Some(Guard {
                            position: (row as isize, column as isize),
                            direction: Direction::North,
                        });
                    }
                    _ => unreachable!(),
                }
            }
        }

        let guard = guard.unwrap();

        let mut this = Self {
            visited: HashMap::default(),
            guard,
            map,
        };
        this.reset(guard);
        this
    }

    fn reset(&mut self, guard: Guard) {
        self.visited = HashMap::from([(guard.position, {
            let mut visited_directions = VisitedDirections::default();
            visited_directions.set_visited(&guard.direction);
            visited_directions
        })]);
        self.guard = guard;
    }

    fn step(&mut self) -> Option<SimulationOutcome> {
        let next_tile = self.guard.looking_at();
        match self.map.get(&next_tile) {
            None => {
                return Some(SimulationOutcome::Exited(self.visited.len()));
            }
            Some(Tile::Floor) => {
                self.guard.step_forward();
            }
            Some(Tile::Obstacle) => {
                self.guard.rotate_right();
            }
        }
        let visited = self.visited.entry(self.guard.position).or_default();
        if visited.get_visited(&self.guard.direction) {
            return Some(SimulationOutcome::Looped);
        }
        visited.set_visited(&self.guard.direction);
        None
    }

    fn run(&mut self) -> SimulationOutcome {
        loop {
            if let Some(outcome) = self.step() {
                return outcome;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Floor,
    Obstacle,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Default, Clone, Copy)]
struct VisitedDirections {
    mask: u8,
}

impl VisitedDirections {
    fn set_visited(&mut self, direction: &Direction) {
        self.mask |= match direction {
            Direction::North => 1u8 << 0,
            Direction::East => 1u8 << 1,
            Direction::South => 1u8 << 2,
            Direction::West => 1u8 << 3,
        };
    }

    fn get_visited(&self, direction: &Direction) -> bool {
        self.mask
            & match direction {
                Direction::North => 1u8 << 0,
                Direction::East => 1u8 << 1,
                Direction::South => 1u8 << 2,
                Direction::West => 1u8 << 3,
            }
            > 0
    }
}

#[derive(Clone, Copy)]
struct Guard {
    position: (isize, isize),
    direction: Direction,
}

impl Guard {
    fn looking_at(&self) -> (isize, isize) {
        match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::East => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
        }
    }

    fn step_forward(&mut self) {
        self.position = self.looking_at();
    }

    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 41);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 6);
    }
}
