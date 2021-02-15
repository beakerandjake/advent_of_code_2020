use std::error::Error;

// Days
pub mod day01;
pub mod day02;
pub mod day03;

pub fn noop(_inp: &str) -> Result<String, Box<dyn Error>> { Ok(String::from("not implemented")) }

pub type DayFn = fn(&str) -> Result<String, Box<dyn Error>>;

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
