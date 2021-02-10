use std::collections::HashSet;
use std::error::Error;

const TARGET_SUM: i32 = 2020;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let entries = parse_entries(&input)?;

    let message = match find_two_that_sum(&entries, TARGET_SUM) {
        None => format!("Could not find two entries that sum to {}", TARGET_SUM),
        Some((lhs, rhs)) => format!("{} * {} = {}", lhs, rhs, lhs*rhs)
    };

    Ok(message)
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

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let entries = parse_entries(&input)?;

    let message = match find_three_that_sum(&entries, TARGET_SUM) {
        None => format!("Could not find three entries that sum to {}", TARGET_SUM),
        Some((a,b,c)) => format!("{} * {} * {} = {}", a, b, c, a*b*c)
    };

    Ok(message)
}

// attempts to find three items in the set which sum to a target number
fn find_three_that_sum(entries: &HashSet<i32>, target: i32) -> Option<(i32, i32, i32)> {
    for entry in entries {
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