use std::collections::HashSet;
use std::error::Error;

const PREAMBLE_LENGTH: usize = 25;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let all_numbers: Vec<i32> = input
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let mut preamble: HashSet<i32> = all_numbers[..PREAMBLE_LENGTH]
        .into_iter()
        .cloned()
        .collect();

    for (index, current) in all_numbers[PREAMBLE_LENGTH..].iter().enumerate() {
        if test_num(*current, &preamble) == false {
            return Ok(current.to_string());
        }

        preamble.remove(&all_numbers[index]);
        preamble.insert(*current);
    }

    Err("not found".into())
}

fn test_num(num: i32, preamble: &HashSet<i32>) -> bool {
    for preamble_num in preamble {
        let needed = num - preamble_num;

        if preamble.contains(&needed) {
            return true;
        }
    }

    return false;
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    Ok("great job".to_string())
}
