mod day01;
mod day02;
mod day03;

fn main() {
    println!("01.1: {}", solve_timed(day01::star_one));
    println!("01.2: {}", solve_timed(day01::star_two));
    println!("02.1: {}", solve_timed(day02::star_one));
    println!("02.2: {}", solve_timed(day02::star_two));
    println!("03.1: {}", solve_timed(day03::star_one));
    println!("03.2: {}", solve_timed(day03::star_two));
}

fn solve_timed<F, O>(f: F) -> String
where
    F: Fn() -> O,
    O: std::fmt::Display,
{
    let now = std::time::SystemTime::now();
    let result = f();
    let elapsed = now.elapsed().unwrap();
    format!("{result} in {elapsed:?}")
}
