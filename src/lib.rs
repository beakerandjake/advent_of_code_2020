use std::env;
use std::error::Error;
use std::fs;
use std::time::Instant;

pub mod config;
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

type DayFn = fn(&str) -> Result<String, Box<dyn Error>>;

// attempts to run both parts of the day
pub fn run_day(day: u32) -> Result<(), Box<dyn Error>> {
    // load text file input for day.
    let input = read_input_file(day)?;
    // load the actual fn's that will be executed.
    let day_fns = get_day(day).ok_or("day not implemented")?;

    // run part 1
    println!("running part 1");
    time_and_run(day_fns.0, &input)?;

    // run part 2
    println!("\nrunning part 2");
    time_and_run(day_fns.1, &input)
}

// load the txt input file for the day and returns the contents.
fn read_input_file(day: u32) -> Result<String, Box<dyn Error>> {
    let file_name = env::current_dir()?
        .join("inputs")
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(file_name).map_err(|e| e.into())
}

// returns the part 1 / 2 functions to execute for the day
fn get_day(day: u32) -> Option<(DayFn, DayFn)> {
    match day {
        1 => Some((day01::part1, day01::part2)),
        2 => Some((day02::part1, day02::part2)),
        3 => Some((day03::part1, day03::part2)),
        4 => Some((day04::part1, day04::part2)),
        5 => Some((day05::part1, day05::part2)),
        6 => Some((day06::part1, day06::part2)),
        7 => Some((day07::part1, day07::part2)),
        8 => Some((day08::part1, day08::part2)),
        9 => Some((day09::part1, day09::part2)),
        10 => Some((day10::part1, day10::part2)),
        _ => None,
    }
}

// run the day fn and measure the amount of time it takes
fn time_and_run(day_fn: DayFn, input: &str) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    day_fn(input).and_then(|message| {
        println!("{}", message);
        let elapsed = start.elapsed();
        println!("took: {:?}", elapsed);
        Ok(())
    })
}
