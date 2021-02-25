use std::env;
use std::process;

use advent_of_code_2020::{run_day, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run_day(config.day) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
