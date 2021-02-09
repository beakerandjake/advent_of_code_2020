use std::collections::HashSet;

const TARGET_SUM: i32 = 2020;

pub fn part1(input: String) {
    match find_two_that_sum(&parse_entries(&input), TARGET_SUM) {
        None => println!("Could not find two entries that sum to {}", TARGET_SUM),
        Some((lhs, rhs)) => println!("{} * {} = {}", lhs, rhs, lhs*rhs)
    };
}


pub fn part2(input: String) {
    let entries = parse_entries(&input);

    for first in entries.iter() {
        for second in entries.iter() {
            let third = TARGET_SUM - first - second;

            if entries.contains(&third) {
                println!("{} * {} * {} = {}", first, second, third, first*second*third);
                return;
            }
        }
    }

    println!("Could not find 3 entries that sum to 2020");
}

// parses each line of the input into a number and collects into a HashSet.  
fn parse_entries(input: &str) -> HashSet<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().expect("expected numeric input"))
        .collect()
}

fn find_two_that_sum(entries: &HashSet<i32>, target: i32) -> Option<(i32,i32)> {
    for entry in entries {
        let need = target - entry;

        if entries.contains(&need) {
            return Some((*entry, need));
        }
    }
    None
}