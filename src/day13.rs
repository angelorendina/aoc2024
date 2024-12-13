use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day13/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day13/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    let mut lines = buffer.lines();
    while let Some(machine) = Machine::parse(&mut lines) {
        total += machine.solve::<false>().unwrap_or_default();
    }

    total
}

pub fn star_two() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day13/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day13/test.txt";

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut total = 0;

    let mut lines = buffer.lines();
    while let Some(machine) = Machine::parse(&mut lines) {
        total += machine.solve::<true>().unwrap_or_default();
    }

    total
}

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn parse<'a, 'b, I>(lines: &'a mut I) -> Option<Self>
    where
        I: Iterator<Item = &'b str>,
        'b: 'a,
    {
        let line = lines.next()?;
        let (_, deltas) = line.split_once("Button A: ")?;
        let (x, y) = deltas.split_once(", ")?;
        let (_, x) = x.split_once('+')?;
        let (_, y) = y.split_once('+')?;
        let a = (x.parse().unwrap(), y.parse().unwrap());

        let line = lines.next()?;
        let (_, deltas) = line.split_once("Button B: ")?;
        let (x, y) = deltas.split_once(", ")?;
        let (_, x) = x.split_once('+')?;
        let (_, y) = y.split_once('+')?;
        let b = (x.parse().unwrap(), y.parse().unwrap());

        let line = lines.next()?;
        let (_, deltas) = line.split_once("Prize: ")?;
        let (x, y) = deltas.split_once(", ")?;
        let (_, x) = x.split_once('=')?;
        let (_, y) = y.split_once('=')?;
        let prize = (x.parse().unwrap(), y.parse().unwrap());

        lines.next();

        Some(Self { a, b, prize })
    }

    fn solve<const EXT: bool>(self) -> Option<u64> {
        let Machine {
            a: (j, k),
            b: (l, m),
            prize: (p, q),
        } = self;

        let (p, q) = match EXT {
            true => (p + 10000000000000, q + 10000000000000),
            false => (p, q),
        };

        // | j l | | u | = | p |
        // | k m | | v | = | q |

        // | u | = 1/  | m -l | | p |
        // | v | = det | -k j | | q |

        let determinant = j * m - l * k;
        let uu = m * p - l * q;
        let vv = -k * p + j * q;
        if uu % determinant == 0 && vv % determinant == 0 {
            let u = uu / determinant;
            let v = vv / determinant;
            if EXT || ((0..=100).contains(&u) && (0..=100).contains(&v)) {
                return Some((u as u64 * 3) + (v as u64));
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
        assert_eq!(star_one(), 480);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 875318608908);
    }
}
