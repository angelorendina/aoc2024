use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day18/input.txt";
    #[cfg(not(test))]
    const ROWS: isize = 71;
    #[cfg(not(test))]
    const COLUMNS: isize = 71;
    #[cfg(not(test))]
    const STEPS: isize = 1024;

    #[cfg(test)]
    const INPUT: &str = "data/day18/test.txt";
    #[cfg(test)]
    const ROWS: isize = 7;
    #[cfg(test)]
    const COLUMNS: isize = 7;
    #[cfg(test)]
    const STEPS: usize = 12;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut map = (0..ROWS)
        .flat_map(|row| (0..COLUMNS).map(move |column| ((row, column), Tile::Safe)))
        .collect::<HashMap<_, _>>();

    for line in buffer.lines().take(STEPS as usize) {
        let (column, row) = line.split_once(',').unwrap();
        let column = column.parse::<isize>().unwrap();
        let row = row.parse::<isize>().unwrap();
        map.insert((row, column), Tile::Corrupted);
    }

    let costs = compute_paths(&map, (0, 0)).unwrap();

    *costs.get(&(ROWS - 1, COLUMNS - 1)).unwrap()
}

pub fn star_two() -> String {
    #[cfg(not(test))]
    const INPUT: &str = "data/day18/input.txt";
    #[cfg(not(test))]
    const ROWS: isize = 71;
    #[cfg(not(test))]
    const COLUMNS: isize = 71;

    #[cfg(test)]
    const INPUT: &str = "data/day18/test.txt";
    #[cfg(test)]
    const ROWS: isize = 7;
    #[cfg(test)]
    const COLUMNS: isize = 7;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut map = (0..ROWS)
        .flat_map(|row| (0..COLUMNS).map(move |column| ((row, column), Tile::Safe)))
        .collect::<HashMap<_, _>>();

    let lines = buffer
        .lines()
        .map(|line| {
            let (column, row) = line.split_once(',').unwrap();
            let column = column.parse::<isize>().unwrap();
            let row = row.parse::<isize>().unwrap();
            (row, column)
        })
        .enumerate()
        .collect::<Vec<_>>();

    let &(_, (column, row)) = binary_search_first(&lines, &mut |&(i, _)| {
        for &(_, coords) in lines.iter().take(i) {
            map.insert(coords, Tile::Corrupted);
        }

        let mut costs = compute_paths(&map, (0, 0)).unwrap();
        let unreachable = *costs.entry((ROWS - 1, COLUMNS - 1)).or_insert(u64::MAX) == u64::MAX;

        for &(_, coords) in lines.iter().take(i) {
            map.insert(coords, Tile::Safe);
        }

        unreachable
    });
    format!("{row},{column}")
}

#[derive(Clone, Copy)]
enum Tile {
    Safe,
    Corrupted,
}

fn compute_paths(
    map: &HashMap<(isize, isize), Tile>,
    start: (isize, isize),
) -> Option<HashMap<(isize, isize), u64>> {
    if let Some(Tile::Safe) = map.get(&start) {
    } else {
        return None;
    }

    let mut costs = HashMap::from([(start, 0)]);

    let mut exploration = VecDeque::from([(start, 0)]);
    while let Some(((row, column), current_cost)) = exploration.pop_front() {
        for neigh in [
            (row - 1, column),
            (row + 1, column),
            (row, column - 1),
            (row, column + 1),
        ] {
            if let Some(Tile::Safe) = map.get(&neigh) {
                let cached_neigh_cost = costs.entry(neigh).or_insert(u64::MAX);
                if current_cost + 1 < *cached_neigh_cost {
                    *cached_neigh_cost = current_cost + 1;
                    exploration.push_back((neigh, current_cost + 1));
                }
            }
        }
    }

    Some(costs)
}

fn binary_search_first<'a, 'b, T>(data: &'a [T], f: &'b mut impl FnMut(&'a T) -> bool) -> &'a T {
    let mut left = 0;
    let mut right = data.len() - 1;

    loop {
        let mid = (right + left) / 2;
        let mid_data = data.get(mid).unwrap();
        let mid_true = f(mid_data);

        if right == left + 1 {
            return mid_data;
        }

        if mid_true {
            right = mid;
        } else {
            left = mid;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 22);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), "6,1");
    }
}
