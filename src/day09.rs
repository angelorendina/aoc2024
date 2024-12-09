use std::{collections::VecDeque, io::Read};

pub fn star_one() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day09/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day09/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut defragmented = VecDeque::new();

    for (i, c) in buffer.lines().next().unwrap().chars().enumerate() {
        let index = i / 2;
        let size = c.to_digit(10).unwrap();
        let is_padding = i % 2 > 0;
        for _ in 0..size {
            defragmented.push_back(if is_padding { None } else { Some(index) });
        }
    }

    let mut total = 0;

    'defrag: for index in 0.. {
        let Some(left_digit) = defragmented.pop_front() else {
            break 'defrag;
        };
        if let Some(left_digit) = left_digit {
            total += index * left_digit;
        } else {
            loop {
                let Some(right_digit) = defragmented.pop_back() else {
                    break 'defrag;
                };
                if let Some(right_digit) = right_digit {
                    total += index * right_digit;
                    break;
                }
            }
        }
    }

    total
}

pub fn star_two() -> usize {
    #[cfg(not(test))]
    const INPUT: &str = "data/day09/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day09/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    #[derive(Debug)]
    enum Block {
        Padding(usize),
        File { id: usize, size: usize },
    }

    let mut fragmented = VecDeque::new();

    for (i, c) in buffer.lines().next().unwrap().chars().enumerate() {
        let id = i / 2;
        let size = c.to_digit(10).unwrap() as usize;
        let is_padding = i % 2 > 0;
        if is_padding {
            fragmented.push_back(Block::Padding(size));
        } else {
            fragmented.push_back(Block::File { id, size });
        }
    }

    let mut defragmented = VecDeque::new();

    'defrag: while let Some(right_block) = fragmented.pop_back() {
        let (id, size) = match right_block {
            pad @ Block::Padding(_) => {
                defragmented.push_front(pad);
                continue 'defrag;
            }
            Block::File { id, size } => (id, size),
        };

        for position in 0..fragmented.len() {
            let pad = match &fragmented[position] {
                Block::Padding(pad) => *pad,
                Block::File { .. } => {
                    continue;
                }
            };

            if let Some(remaining_padding) = pad.checked_sub(size) {
                if remaining_padding == 0 {
                    fragmented[position] = Block::File { id, size };
                } else {
                    fragmented[position] = Block::Padding(remaining_padding);
                    fragmented.insert(position, Block::File { id, size });
                }
                defragmented.push_front(Block::Padding(size));
                continue 'defrag;
            }
        }

        defragmented.push_front(Block::File { id, size });
    }

    let mut total = 0;

    let mut position = 0;
    for block in defragmented {
        match block {
            Block::Padding(pad) => {
                position += pad;
            }
            Block::File { id, size } => {
                for _ in 0..size {
                    total += position * id;
                    position += 1;
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
        assert_eq!(star_one(), 1928);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 2858);
    }
}
