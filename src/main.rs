mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

fn main() {
    println!("01.1: {}", solve_timed(day01::star_one));
    println!("01.2: {}", solve_timed(day01::star_two));
    println!("02.1: {}", solve_timed(day02::star_one));
    println!("02.2: {}", solve_timed(day02::star_two));
    println!("03.1: {}", solve_timed(day03::star_one));
    println!("03.2: {}", solve_timed(day03::star_two));
    println!("04.1: {}", solve_timed(day04::star_one));
    println!("04.2: {}", solve_timed(day04::star_two));
    println!("05.1: {}", solve_timed(day05::star_one));
    println!("05.2: {}", solve_timed(day05::star_two));
    println!("06.1: {}", solve_timed(day06::star_one));
    println!("06.2: {}", solve_timed(day06::star_two));
    println!("07.1: {}", solve_timed(day07::star_one));
    println!("07.2: {}", solve_timed(day07::star_two));
    println!("08.1: {}", solve_timed(day08::star_one));
    println!("08.2: {}", solve_timed(day08::star_two));
    println!("09.1: {}", solve_timed(day09::star_one));
    println!("09.2: {}", solve_timed(day09::star_two));
    println!("10.1: {}", solve_timed(day10::star_one));
    println!("10.2: {}", solve_timed(day10::star_two));
    println!("11.1: {}", solve_timed(day11::star_one));
    println!("11.2: {}", solve_timed(day11::star_two));
    println!("12.1: {}", solve_timed(day12::star_one));
    println!("12.2: {}", solve_timed(day12::star_two));
    println!("13.1: {}", solve_timed(day13::star_one));
    println!("13.2: {}", solve_timed(day13::star_two));
    println!("14.1: {}", solve_timed(day14::star_one));
    println!("14.2: {}", solve_timed(day14::star_two));
    println!("15.1: {}", solve_timed(day15::star_one));
    println!("15.2: {}", solve_timed(day15::star_two));
    println!("16.1: {}", solve_timed(day16::star_one));
    println!("16.2: {}", solve_timed(day16::star_two));
    println!("17:1: {}", solve_timed(day17::star_one));
    println!("17:2: {}", solve_timed(day17::star_two));
    println!("18:1: {}", solve_timed(day18::star_one));
    println!("18:2: {}", solve_timed(day18::star_two));
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
