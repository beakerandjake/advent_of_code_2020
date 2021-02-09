use std::collections::HashSet;


pub fn part1(input: String) {

    let entries: HashSet<i32> = input
        .lines()
        .map(|line| line.trim().parse().expect("expected numeric input"))
        .collect();

    for entry in entries.iter() {
        let needed = 2020 - entry;

        if entries.contains(&needed) {
            println!("{} + {} = {}", entry, needed, entry*needed);
            return;
        }
    }

    println!("Could not find entries that sum to 2020");
}

