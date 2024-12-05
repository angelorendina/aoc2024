use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day04/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day04/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let grid = buffer
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns = grid[0].len();
    let rows = grid.len();

    let mut total = 0;

    for r in 0..rows {
        for c in 0..columns {
            let xmas = |letters: [char; 4]| match letters {
                ['X', 'M', 'A', 'S'] => 1,
                ['S', 'A', 'M', 'X'] => 1,
                _ => 0,
            };

            if c < columns - 3 {
                let horizontal = std::array::from_fn::<_, 4, _>(|i| grid[r][c + i]);
                total += xmas(horizontal);
            }

            if r < rows - 3 {
                let vertical = std::array::from_fn::<_, 4, _>(|i| grid[r + i][c]);
                total += xmas(vertical);
            }

            if c < columns - 3 && r < rows - 3 {
                let diagonal = std::array::from_fn::<_, 4, _>(|i| grid[r + i][c + i]);
                total += xmas(diagonal);
            }

            if c < columns - 3 && r < rows - 3 {
                let antidiagonal = std::array::from_fn::<_, 4, _>(|i| grid[r + i][c + 3 - i]);
                total += xmas(antidiagonal);
            }
        }
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day04/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day04/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let grid = buffer
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let columns = grid[0].len();
    let rows = grid.len();

    let mut total = 0;

    for r in 1..(rows - 1) {
        for c in 1..(columns - 1) {
            let mas = |letters: [char; 3]| matches!(letters, ['M', 'A', 'S'] | ['S', 'A', 'M']);

            let diagonal = std::array::from_fn::<_, 3, _>(|i| grid[r - 1 + i][c - 1 + i]);
            if !mas(diagonal) {
                continue;
            }

            let antidiagonal = std::array::from_fn::<_, 3, _>(|i| grid[r - 1 + i][c + 1 - i]);
            if !mas(antidiagonal) {
                continue;
            }

            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 18);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 9);
    }
}
