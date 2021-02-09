use std::collections::HashSet;

const targetSum: i32 = 2020;

pub fn part1(input: String) {

    let entries = parse_entries(&input);

    for entry in entries.iter() {
        let needed = targetSum - entry;

        if entries.contains(&needed) {
            println!("{} * {} = {}", entry, needed, entry*needed);
            return;
        }
    }

    println!("Could not find entries that sum to 2020");
}


pub fn part2(input: String) {
    let entries = parse_entries(&input);

    for first in entries.iter() {
        for second in entries.iter() {
            let third = targetSum - first - second;

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