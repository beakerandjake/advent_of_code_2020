use std::collections::HashSet;
use std::error::Error;

const TARGET_SUM: i32 = 2020;

pub fn part1(input: String) -> Result<String, Box<dyn Error>> {
    let parsed = parse_entries(&input)?;

    match find_two_that_sum(&parsed, TARGET_SUM) {
        None => Err(format!("Could not find two entries that sum to {}", TARGET_SUM)),
        Some((lhs, rhs)) => Ok(format!("{} * {} = {}", lhs, rhs, lhs*rhs))
    }
}

// attempts to find two items in the set which sum to a target number
fn find_two_that_sum(entries: &HashSet<i32>, target: i32) -> Option<(i32,i32)> {
    for entry in entries {
        let need = target - entry;

        if entries.contains(&need) {
            return Some((*entry, need));
        }
    }
    None
}

pub fn part2(input: String) {
    match find_three_that_sum(&parse_entries(&input), TARGET_SUM) {
        None => println!("Could not find three entries that sum to {}", TARGET_SUM),
        Some((a,b,c)) => println!("{} * {} * {} = {}", a, b, c, a*b*c)
    };
}


// attempts to find three items in the set which sum to a target number
fn find_three_that_sum(entries: &HashSet<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for entry in entries.iter() {
        let target = target - entry;

        if let Some((lhs,rhs)) = find_two_that_sum(&entries, target) {
            return Some((*entry, lhs, rhs));
        }
    }
    None
}

// parses each line of the input string and creates a HashSet   
fn parse_entries(input: &str) -> Result<HashSet<i32>, std::num::ParseIntError> {
    input
        .lines()
        .map(|line| line.trim().parse())
        .collect()
}