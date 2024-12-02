mod day01;

fn main() {
    println!("01.1: {}", solve_timed(day01::star_one));
    println!("01.2: {}", solve_timed(day01::star_two));
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
