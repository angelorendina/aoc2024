use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day21/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day21/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    solve_at_depth(&buffer, 1)
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day21/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day21/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    solve_at_depth(&buffer, 24)
}

fn solve_at_depth(input: &str, depth: usize) -> usize {
    // cost at each depth for moving the arm to "just pressed X" to "just pressed Y"
    // i.e. cost of moving the arm from X to Y then pushing it
    // depth 0 is the human-controlled
    let mut level_cost = HashMap::new();
    for start in [UP, DOWN, LEFT, RIGHT, A] {
        for end in [UP, DOWN, LEFT, RIGHT, A] {
            let mut cheapest_path_cost = usize::MAX;
            for path in Paths::new_for_keypad(start, end) {
                let cost = path.len() + 1;
                cheapest_path_cost = usize::min(cheapest_path_cost, cost);
            }
            level_cost.insert((0, start, end), cheapest_path_cost);
        }
    }

    // recursively compute deeper nesting levels
    for level in 1..=depth {
        for start in [UP, DOWN, LEFT, RIGHT, A] {
            for end in [UP, DOWN, LEFT, RIGHT, A] {
                let mut cheapest = usize::MAX;
                for instructions in Paths::new_for_keypad(start, end) {
                    let mut cost = 0;
                    let mut at = A;
                    for instruction in instructions {
                        cost += level_cost[&(level - 1, at, instruction)];
                        at = instruction;
                    }
                    cost += level_cost[&(level - 1, at, A)];
                    cheapest = usize::min(cheapest, cost)
                }
                level_cost.insert((level, start, end), cheapest);
            }
        }
    }

    let mut total = 0;

    for line in input.lines() {
        let line_value = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();

        let line = line
            .chars()
            .map(|c| match c {
                'A' => A,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let mut line_cost = 0;

        let mut number_at = A;
        for number_to in line {
            let mut cheapest = usize::MAX;
            for instruction_set in Paths::new_for_numpad(number_at, number_to) {
                let mut cost = 0;
                let mut control_at = A;
                for instruction in instruction_set {
                    cost += level_cost[&(depth, control_at, instruction)];
                    control_at = instruction;
                }
                cost += level_cost[&(depth, control_at, A)];
                cheapest = usize::min(cheapest, cost);
            }
            line_cost += cheapest;
            number_at = number_to;
        }

        total += line_cost * line_value;
    }

    total
}

type Button = u8;
type Direction = u8;
type Coords = (isize, isize);

const UP: Button = 10;
const LEFT: Button = 11;
const DOWN: Button = 12;
const RIGHT: Button = 13;
const A: Button = 100;
// const INVALID: Button = 255;

// const NUMPAD: [[Button; 3]; 4] = [[7, 8, 9], [4, 5, 6], [1, 2, 3], [INVALID, 0, A]];
// const KEYPAD: [[Button; 3]; 2] = [[INVALID, UP, A], [LEFT, DOWN, RIGHT]];

const fn numpad_at(coords: Coords) -> Option<Button> {
    match coords {
        (3, 1) => Some(0),
        (2, 0) => Some(1),
        (2, 1) => Some(2),
        (2, 2) => Some(3),
        (1, 0) => Some(4),
        (1, 1) => Some(5),
        (1, 2) => Some(6),
        (0, 0) => Some(7),
        (0, 1) => Some(8),
        (0, 2) => Some(9),
        (3, 2) => Some(A),
        _ => None,
    }
}

const fn numpad_where(num: Button) -> Option<Coords> {
    match num {
        0 => Some((3, 1)),
        1 => Some((2, 0)),
        2 => Some((2, 1)),
        3 => Some((2, 2)),
        4 => Some((1, 0)),
        5 => Some((1, 1)),
        6 => Some((1, 2)),
        7 => Some((0, 0)),
        8 => Some((0, 1)),
        9 => Some((0, 2)),
        A => Some((3, 2)),
        _ => None,
    }
}

const fn keypad_at(coords: Coords) -> Option<Button> {
    match coords {
        (0, 1) => Some(UP),
        (1, 0) => Some(LEFT),
        (1, 1) => Some(DOWN),
        (1, 2) => Some(RIGHT),
        (0, 2) => Some(A),
        _ => None,
    }
}

const fn keypad_where(key: Button) -> Option<Coords> {
    match key {
        UP => Some((0, 1)),
        LEFT => Some((1, 0)),
        DOWN => Some((1, 1)),
        RIGHT => Some((1, 2)),
        A => Some((0, 2)),
        _ => None,
    }
}

struct Paths {
    start: Button,
    end: Button,
    inner: VecDeque<Vec<Button>>,
    button_to_coords: fn(Button) -> Option<Coords>,
    coords_to_button: fn(Coords) -> Option<Button>,
}

impl Paths {
    fn new_for_numpad(start: Button, end: Button) -> Self {
        assert!(numpad_where(start).is_some());
        assert!(numpad_where(end).is_some());
        Self {
            start,
            end,
            inner: VecDeque::from([vec![]]),
            button_to_coords: numpad_where,
            coords_to_button: numpad_at,
        }
    }

    fn new_for_keypad(start: Button, end: Button) -> Self {
        assert!(keypad_where(start).is_some());
        assert!(keypad_where(end).is_some());
        Self {
            start,
            end,
            inner: VecDeque::from([vec![]]),
            button_to_coords: keypad_where,
            coords_to_button: keypad_at,
        }
    }
}

impl Iterator for Paths {
    type Item = Vec<Direction>;

    fn next(&mut self) -> Option<Self::Item> {
        let start_coords = (self.button_to_coords)(self.start).unwrap();
        let end_coords = (self.button_to_coords)(self.end).unwrap();
        let max_steps = isize::abs_diff(start_coords.0, end_coords.0)
            + isize::abs_diff(start_coords.1, end_coords.1);

        while let Some(path) = self.inner.pop_front() {
            if path.len() > max_steps {
                continue;
            }
            let mut coords_at = start_coords;
            for &direction in &path {
                match direction {
                    UP => {
                        coords_at.0 -= 1;
                    }
                    DOWN => {
                        coords_at.0 += 1;
                    }
                    LEFT => {
                        coords_at.1 -= 1;
                    }
                    RIGHT => {
                        coords_at.1 += 1;
                    }
                    _ => unreachable!(),
                }
            }
            let Some(button_at) = (self.coords_to_button)(coords_at) else {
                continue;
            };
            if button_at == self.end {
                return Some(path);
            }

            if (self.coords_to_button)((coords_at.0 - 1, coords_at.1)).is_some()
                && (path.last() == Some(&UP) || !path.contains(&UP))
            {
                let mut path = path.clone();
                path.push(UP);
                self.inner.push_back(path);
            }

            if (self.coords_to_button)((coords_at.0, coords_at.1 - 1)).is_some()
                && (path.last() == Some(&LEFT) || !path.contains(&LEFT))
            {
                let mut path = path.clone();
                path.push(LEFT);
                self.inner.push_back(path);
            }

            if (self.coords_to_button)((coords_at.0 + 1, coords_at.1)).is_some()
                && (path.last() == Some(&DOWN) || !path.contains(&DOWN))
            {
                let mut path = path.clone();
                path.push(DOWN);
                self.inner.push_back(path);
            }

            if (self.coords_to_button)((coords_at.0, coords_at.1 + 1)).is_some()
                && (path.last() == Some(&RIGHT) || !path.contains(&RIGHT))
            {
                let mut path = path;
                path.push(RIGHT);
                self.inner.push_back(path);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 126384);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 154115708116294);
    }
}
