use std::error::Error;
use std::collections::HashSet;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    
    let mut total_count = 0;

    for group in input.split("\r\n\r\n") {
        let mut unique_answers = HashSet::new();

        for person in group.lines() {
            for answer in person.chars() {
                unique_answers.insert(answer);
            }
        }

        total_count += unique_answers.len();
    }

    Ok(total_count.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}


