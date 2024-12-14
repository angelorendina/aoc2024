use std::io::Read;

pub fn star_one() -> u64 {
    #[cfg(not(test))]
    const INPUT: &str = "data/day14/input.txt";
    #[cfg(test)]
    const INPUT: &str = "data/day14/test.txt";

    #[cfg(not(test))]
    const WIDTH: isize = 101;
    #[cfg(test)]
    const WIDTH: isize = 11;

    #[cfg(not(test))]
    const HEIGHT: isize = 103;
    #[cfg(test)]
    const HEIGHT: isize = 7;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut upright = 0;
    let mut upleft = 0;
    let mut downright = 0;
    let mut downleft = 0;

    for line in buffer.lines() {
        let (position, velocity) = line.split_once(' ').unwrap();
        let (_, position) = position.split_once('=').unwrap();
        let position = position.split_once(',').unwrap();
        let position = (
            position.0.parse::<isize>().unwrap(),
            position.1.parse::<isize>().unwrap(),
        );
        let (_, velocity) = velocity.split_once('=').unwrap();
        let velocity = velocity.split_once(',').unwrap();
        let velocity = (
            velocity.0.parse::<isize>().unwrap(),
            velocity.1.parse::<isize>().unwrap(),
        );

        let x = (position.0 + velocity.0 * 100).rem_euclid(WIDTH);
        let y = (position.1 + velocity.1 * 100).rem_euclid(HEIGHT);

        if x < WIDTH / 2 && y < HEIGHT / 2 {
            upleft += 1;
        }
        if x < WIDTH / 2 && y > HEIGHT / 2 {
            downleft += 1;
        }
        if x > WIDTH / 2 && y < HEIGHT / 2 {
            upright += 1;
        }
        if x > WIDTH / 2 && y > HEIGHT / 2 {
            downright += 1;
        }
    }

    upleft * upright * downleft * downright
}

pub fn star_two() -> u64 {
    const INPUT: &str = "data/day14/input.txt";
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;

    let mut buffer = String::new();
    std::fs::File::open(INPUT)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut bots = buffer
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(' ').unwrap();
            let (_, position) = position.split_once('=').unwrap();
            let position = position.split_once(',').unwrap();
            let position = (
                position.0.parse::<isize>().unwrap(),
                position.1.parse::<isize>().unwrap(),
            );
            let (_, velocity) = velocity.split_once('=').unwrap();
            let velocity = velocity.split_once(',').unwrap();
            let velocity = (
                velocity.0.parse::<isize>().unwrap(),
                velocity.1.parse::<isize>().unwrap(),
            );

            (position, velocity)
        })
        .collect::<Vec<_>>();

    let n = bots.len() as f64;

    let mut min_var = f64::MAX;
    let mut smallest_time = None;

    // find distribution of x and y that minimises variance
    // also the pattern repeats at most every WIDTH * HEIGHT due to chinese remainder theorem
    for time in 0..=(WIDTH as u64 * HEIGHT as u64) {
        let x_mean = bots
            .iter()
            .map(|((x, _), _)| f64::from(*x as u8))
            .sum::<f64>()
            / n;
        let x_square_mean = bots
            .iter()
            .map(|((x, _), _)| f64::from(*x as u8).powi(2))
            .sum::<f64>()
            / n;
        let x_var = x_square_mean - x_mean.powi(2);

        let y_mean = bots
            .iter()
            .map(|((_, y), _)| f64::from(*y as u8))
            .sum::<f64>()
            / n;
        let y_square_mean = bots
            .iter()
            .map(|((_, y), _)| f64::from(*y as u8).powi(2))
            .sum::<f64>()
            / n;
        let y_var = y_square_mean - y_mean.powi(2);

        let var = x_var * y_var;
        if var < min_var {
            smallest_time = Some(time);
            min_var = var;
        }

        for (p, v) in bots.iter_mut() {
            p.0 = (p.0 + v.0).rem_euclid(WIDTH);
            p.1 = (p.1 + v.1).rem_euclid(HEIGHT);
        }
    }

    smallest_time.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_star() {
        assert_eq!(star_one(), 12);
    }

    #[test]
    fn second_star() {
        assert_eq!(star_two(), 8179);
    }
}
