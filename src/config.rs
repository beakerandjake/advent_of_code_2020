use std::error::Error;
use std::env;

pub struct Config {
    pub day: u32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        // consume first arg (name of the program)
        args.next();

        // expect next arg is the day number
        let day: u32 = args
            .next()
            .ok_or("day is missing")?
            .parse()
            .or(Err("day was not numeric"))?;

        Ok(Config { day })
    }
}