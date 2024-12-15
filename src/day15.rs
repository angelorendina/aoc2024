use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day15/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day15/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut simulation = Simulation::parse(&buffer);

    while !simulation.actions.is_empty() {
        simulation.step();
    }

    simulation.score()
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day15/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day15/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut simulation = LargeSimulation::parse(&buffer);

    while !simulation.actions.is_empty() {
        simulation.step();
    }

    simulation.score()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coords {
    row: isize,
    column: isize,
}

impl Coords {
    fn looking_into(self, direction: Direction) -> Self {
        Self {
            row: self.row
                + match direction {
                    Direction::Up => -1,
                    Direction::Down => 1,
                    Direction::Left => 0,
                    Direction::Right => 0,
                },
            column: self.column
                + match direction {
                    Direction::Up => 0,
                    Direction::Down => 0,
                    Direction::Left => -1,
                    Direction::Right => 1,
                },
        }
    }
}

struct Simulation {
    map: HashMap<Coords, Object>,
    fish: Coords,
    actions: VecDeque<Direction>,
}

impl Simulation {
    fn parse(input: &str) -> Self {
        let fish = std::cell::OnceCell::new();

        let map = input
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row, line)| {
                let fish = &fish;
                line.chars().enumerate().map(move |(column, c)| {
                    let coords = Coords {
                        row: row as isize,
                        column: column as isize,
                    };

                    let object = match c {
                        '@' => {
                            fish.get_or_init(|| coords);
                            Object::Empty
                        }
                        '.' => Object::Empty,
                        '#' => Object::Wall,
                        'O' => Object::Box,
                        _ => unreachable!(),
                    };

                    (coords, object)
                })
            })
            .collect();

        let actions = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .flat_map(|line| line.chars())
            .fold(VecDeque::new(), |mut actions, c| {
                actions.push_back(match c {
                    'v' => Direction::Down,
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                });
                actions
            });

        Self {
            map,
            fish: fish.into_inner().unwrap(),
            actions,
        }
    }

    fn push_then_is_empty(&mut self, coords: Coords, direction: Direction) -> bool {
        let target_coords = coords.looking_into(direction);
        let target_object = *self.map.get(&target_coords).unwrap();
        match target_object {
            Object::Empty => true,
            Object::Wall => false,
            Object::Box => {
                let target_free_after_push = self.push_then_is_empty(target_coords, direction);
                if target_free_after_push {
                    self.map.insert(target_coords, Object::Empty);
                    self.map
                        .insert(target_coords.looking_into(direction), Object::Box);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn step(&mut self) {
        if let Some(direction) = self.actions.pop_front() {
            let free_after_push = self.push_then_is_empty(self.fish, direction);
            if free_after_push {
                self.fish = self.fish.looking_into(direction);
            }
        }
    }

    fn score(&self) -> u64 {
        self.map
            .iter()
            .filter_map(|(coords, object)| matches!(object, Object::Box).then_some(*coords))
            .map(|coords| coords.row as u64 * 100 + coords.column as u64)
            .sum()
    }
}

#[derive(Clone, Copy)]
enum LargeObject {
    Empty,
    Wall,
    LeftBox,
    RightBox,
}

struct LargeSimulation {
    map: HashMap<Coords, LargeObject>,
    fish: Coords,
    actions: VecDeque<Direction>,
}

impl LargeSimulation {
    fn parse(input: &str) -> Self {
        let fish = std::cell::OnceCell::new();

        let map = input
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(row, line)| {
                let fish = &fish;
                line.chars().enumerate().flat_map(move |(column, c)| {
                    let coords = Coords {
                        row: row as isize,
                        column: column as isize * 2,
                    };
                    let next_coords = Coords {
                        row: coords.row,
                        column: coords.column + 1,
                    };

                    match c {
                        '@' => {
                            fish.get_or_init(|| coords);
                            [
                                (coords, LargeObject::Empty),
                                (next_coords, LargeObject::Empty),
                            ]
                        }
                        '.' => [
                            (coords, LargeObject::Empty),
                            (next_coords, LargeObject::Empty),
                        ],
                        '#' => [
                            (coords, LargeObject::Wall),
                            (next_coords, LargeObject::Wall),
                        ],
                        'O' => [
                            (coords, LargeObject::LeftBox),
                            (next_coords, LargeObject::RightBox),
                        ],
                        _ => unreachable!(),
                    }
                })
            })
            .collect();

        let actions = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .flat_map(|line| line.chars())
            .fold(VecDeque::new(), |mut actions, c| {
                actions.push_back(match c {
                    'v' => Direction::Down,
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                });
                actions
            });

        Self {
            map,
            fish: fish.into_inner().unwrap(),
            actions,
        }
    }

    fn push_then_is_empty(&mut self, coords: Coords, direction: Direction, do_move: bool) -> bool {
        let target_coords = coords.looking_into(direction);
        let target_object = *self.map.get(&target_coords).unwrap();
        match target_object {
            LargeObject::Empty => true,
            LargeObject::Wall => false,
            LargeObject::LeftBox => match direction {
                Direction::Up | Direction::Down => {
                    let left_could_move = self.push_then_is_empty(target_coords, direction, false);
                    let right_could_move = self.push_then_is_empty(
                        target_coords.looking_into(Direction::Right),
                        direction,
                        false,
                    );
                    if left_could_move && right_could_move {
                        if do_move {
                            // push subsequent objects first
                            self.push_then_is_empty(target_coords, direction, true);
                            self.push_then_is_empty(
                                target_coords.looking_into(Direction::Right),
                                direction,
                                true,
                            );
                            // then push target
                            self.map.insert(target_coords, LargeObject::Empty);
                            self.map.insert(
                                target_coords.looking_into(Direction::Right),
                                LargeObject::Empty,
                            );
                            self.map.insert(
                                target_coords.looking_into(direction),
                                LargeObject::LeftBox,
                            );
                            self.map.insert(
                                target_coords
                                    .looking_into(direction)
                                    .looking_into(Direction::Right),
                                LargeObject::RightBox,
                            );
                        }
                        true
                    } else {
                        false
                    }
                }
                Direction::Left | Direction::Right => {
                    let target_free_after_push =
                        self.push_then_is_empty(target_coords, direction, true);
                    if target_free_after_push {
                        if do_move {
                            self.map.insert(target_coords, LargeObject::Empty);
                            self.map.insert(
                                target_coords.looking_into(direction),
                                LargeObject::LeftBox,
                            );
                        }
                        true
                    } else {
                        false
                    }
                }
            },
            LargeObject::RightBox => match direction {
                Direction::Up | Direction::Down => {
                    let right_could_move = self.push_then_is_empty(target_coords, direction, false);
                    let left_could_move = self.push_then_is_empty(
                        target_coords.looking_into(Direction::Left),
                        direction,
                        false,
                    );
                    if left_could_move && right_could_move {
                        if do_move {
                            // push subsequent objects first
                            self.push_then_is_empty(target_coords, direction, true);
                            self.push_then_is_empty(
                                target_coords.looking_into(Direction::Left),
                                direction,
                                true,
                            );
                            // then push target
                            self.map.insert(target_coords, LargeObject::Empty);
                            self.map.insert(
                                target_coords.looking_into(Direction::Left),
                                LargeObject::Empty,
                            );
                            self.map.insert(
                                target_coords.looking_into(direction),
                                LargeObject::RightBox,
                            );
                            self.map.insert(
                                target_coords
                                    .looking_into(direction)
                                    .looking_into(Direction::Left),
                                LargeObject::LeftBox,
                            );
                        }
                        true
                    } else {
                        false
                    }
                }
                Direction::Left | Direction::Right => {
                    let target_free_after_push =
                        self.push_then_is_empty(target_coords, direction, true);
                    if target_free_after_push {
                        if do_move {
                            self.map.insert(target_coords, LargeObject::Empty);
                            self.map.insert(
                                target_coords.looking_into(direction),
                                LargeObject::RightBox,
                            );
                        }
                        true
                    } else {
                        false
                    }
                }
            },
        }
    }

    fn step(&mut self) {
        if let Some(direction) = self.actions.pop_front() {
            let free_after_push = self.push_then_is_empty(self.fish, direction, true);
            if free_after_push {
                self.fish = self.fish.looking_into(direction);
            }
        }
    }

    fn score(&self) -> u64 {
        self.map
            .iter()
            .filter_map(|(coords, object)| {
                matches!(object, LargeObject::LeftBox).then_some(*coords)
            })
            .map(|coords| coords.row as u64 * 100 + coords.column as u64)
            .sum()
    }
}

#[derive(Clone, Copy)]
enum Object {
    Wall,
    Box,
    Empty,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 10092);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 9021);
    }
}
