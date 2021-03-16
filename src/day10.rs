use std::collections::HashSet;
use std::error::Error;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut adapters: HashSet<u32> = input
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut current = 0;
    let mut adds = [(1, 0), (2, 0), (3, 1)];

    while adapters.is_empty() == false {
        adapters.remove(&current);
        for add in adds.iter_mut() {
            let needed = current + add.0;
            if adapters.contains(&needed) {
                current = needed;
                add.1 += 1;
                break;
            }
        }
    }

    Ok((adds[0].1 * adds[2].1).to_string())
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    Ok("great job".into())
}
