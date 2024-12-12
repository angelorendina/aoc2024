use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day12/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day12/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let map = buffer
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, c)| ((row as isize, column as isize), c))
        })
        .collect::<HashMap<_, _>>();

    let mut coloured_map = map
        .keys()
        .map(|tile| (*tile, 0u64))
        .collect::<HashMap<_, _>>();
    let mut used_colours = 0;

    while let Some((&tile, _)) = coloured_map.iter().find(|(_, i)| i == &&0) {
        used_colours += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            match coloured_map.get_mut(&(row, column)) {
                None => continue,
                Some(c) if *c > 0 => continue,
                Some(c) => *c = used_colours,
            }
            for neigh in [
                (row + 1, column),
                (row - 1, column),
                (row, column + 1),
                (row, column - 1),
            ] {
                if map.get(&tile) == map.get(&neigh) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    let fenced_map = coloured_map
        .iter()
        .map(|(&tile, c)| {
            let mut mask = 0u8;
            for (mask_index, neigh) in [
                (tile.0 + 1, tile.1),
                (tile.0 - 1, tile.1),
                (tile.0, tile.1 + 1),
                (tile.0, tile.1 - 1),
            ]
            .into_iter()
            .enumerate()
            {
                if coloured_map.get(&neigh) != Some(c) {
                    mask |= 1u8 << mask_index;
                }
            }
            (tile, mask)
        })
        .collect::<HashMap<_, _>>();

    let mut by_region = HashMap::new();
    for (tile, fence_mask) in fenced_map {
        let c = coloured_map.get(&tile).copied().unwrap();
        let (area, perimeter) = by_region.entry(c).or_insert((0u64, 0u64));
        *area += 1;
        *perimeter += fence_mask.count_ones() as u64;
    }

    let mut total = 0;
    for (_, (area, perimeter)) in by_region {
        total += area * perimeter;
    }
    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day12/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day12/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let map = buffer
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, c)| ((row as isize, column as isize), c))
        })
        .collect::<HashMap<_, _>>();

    let mut coloured_map = map
        .keys()
        .map(|tile| (*tile, 0u64))
        .collect::<HashMap<_, _>>();
    let mut used_colours = 0;

    while let Some((&tile, _)) = coloured_map.iter().find(|(_, i)| i == &&0) {
        used_colours += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            match coloured_map.get_mut(&(row, column)) {
                None => continue,
                Some(c) if *c > 0 => continue,
                Some(c) => *c = used_colours,
            }
            for neigh in [
                (row + 1, column),
                (row - 1, column),
                (row, column + 1),
                (row, column - 1),
            ] {
                if map.get(&tile) == map.get(&neigh) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    let fenced_map = coloured_map
        .iter()
        .map(|(&tile, c)| {
            let mut mask = 0u8;
            for (mask_index, neigh) in [
                (tile.0 + 1, tile.1),
                (tile.0 - 1, tile.1),
                (tile.0, tile.1 + 1),
                (tile.0, tile.1 - 1),
            ]
            .into_iter()
            .enumerate()
            {
                if coloured_map.get(&neigh) != Some(c) {
                    mask |= 1u8 << mask_index;
                }
            }
            (tile, mask)
        })
        .collect::<HashMap<_, _>>();

    let mut long_fences_map = fenced_map
        .iter()
        .map(|(&tile, &fence)| {
            (
                tile,
                [
                    if fence & 1 > 0 { Some(0) } else { None },
                    if fence & 2 > 0 { Some(0) } else { None },
                    if fence & 4 > 0 { Some(0) } else { None },
                    if fence & 8 > 0 { Some(0) } else { None },
                ],
            )
        })
        .collect::<HashMap<_, _>>();
    let mut used_fence_ids = 0u64;

    // bottom fences
    while let Some((&tile, _)) = long_fences_map.iter().find(|(_, fs)| fs[0] == Some(0)) {
        used_fence_ids += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            long_fences_map.get_mut(&(row, column)).unwrap()[0] = Some(used_fence_ids);

            let neigh = (row, column + 1);

            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[0] == Some(0) {
                    exploration.push_back(neigh);
                }
            }

            let neigh = (row, column - 1);
            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[0] == Some(0) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    // top fences
    while let Some((&tile, _)) = long_fences_map.iter().find(|(_, fs)| fs[1] == Some(0)) {
        used_fence_ids += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            long_fences_map.get_mut(&(row, column)).unwrap()[1] = Some(used_fence_ids);

            let neigh = (row, column + 1);

            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[1] == Some(0) {
                    exploration.push_back(neigh);
                }
            }

            let neigh = (row, column - 1);
            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[1] == Some(0) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    // right fences
    while let Some((&tile, _)) = long_fences_map.iter().find(|(_, fs)| fs[2] == Some(0)) {
        used_fence_ids += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            long_fences_map.get_mut(&(row, column)).unwrap()[2] = Some(used_fence_ids);

            let neigh = (row + 1, column);

            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[2] == Some(0) {
                    exploration.push_back(neigh);
                }
            }

            let neigh = (row - 1, column);
            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[2] == Some(0) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    // right fences
    while let Some((&tile, _)) = long_fences_map.iter().find(|(_, fs)| fs[3] == Some(0)) {
        used_fence_ids += 1;
        let mut exploration = VecDeque::from([tile]);
        while let Some((row, column)) = exploration.pop_front() {
            long_fences_map.get_mut(&(row, column)).unwrap()[3] = Some(used_fence_ids);

            let neigh = (row + 1, column);

            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[3] == Some(0) {
                    exploration.push_back(neigh);
                }
            }

            let neigh = (row - 1, column);
            if let Some(fs) = long_fences_map.get_mut(&neigh) {
                if coloured_map.get(&tile) == coloured_map.get(&neigh) && fs[3] == Some(0) {
                    exploration.push_back(neigh);
                }
            }
        }
    }

    let mut area_by_colour = HashMap::new();
    let mut perimeter_by_colour = HashMap::new();
    let mut fence_ids_and_colours = HashSet::new();
    for &tile in map.keys() {
        let c = coloured_map.get(&tile).copied().unwrap();
        *area_by_colour.entry(c).or_insert(0u64) += 1;
        let fs = long_fences_map.get(&tile).copied().unwrap();
        for f in fs.into_iter().flatten() {
            let added = fence_ids_and_colours.insert((c, f));
            if added {
                *perimeter_by_colour.entry(c).or_insert(0u64) += 1;
            }
        }
    }

    let mut total = 0;
    for (c, area) in area_by_colour {
        let perimeter = perimeter_by_colour[&c];
        total += area * perimeter;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 1930);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 1206);
    }
}
