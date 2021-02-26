use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

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
    let mut total_count = 0;

    for group in input.split("\r\n\r\n") {
        let mut answer_counts = HashMap::new();
        
        for person in group.lines() {
            for answer in person.chars() {
                *answer_counts.entry(answer).or_insert(0) += 1;
            }
        }
        
        // only increment by number of questions that were answered by the whole group.
        let group_count = group.lines().count();
        total_count += answer_counts.values().filter(|&&x| x == group_count).count();
    }

    Ok(total_count.to_string())
}
