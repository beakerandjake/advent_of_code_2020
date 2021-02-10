use std::error::Error;

// Days
pub mod day01;

pub fn noop(_inp: &str) -> Result<String, Box<dyn Error>> { Ok(String::from("not implemented")) }

pub type DayFn = fn(&str) -> Result<String, Box<dyn Error>>;

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        1 => (day01::part1, day01::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
