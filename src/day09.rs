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

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let all_numbers: Vec<i32> = input
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let target = 1492208709;
    let mut sum = 0;
    let mut set: Vec<i32> = Vec::new();

    for number in all_numbers {
        set.push(number);
        sum += number;

        while sum > target {
            sum -= set.remove(0);
        }

        if sum == target {
            return find_weakness(&set)
                .map(|x| x.to_string())
                .ok_or("error finding weakness".into());
        }
    }

    Ok("could not find weakness".to_string())
}

fn find_weakness(set: &[i32]) -> Option<i32> {
    if set.len() <= 1 {
        return None;
    }

    let mut sorted = set.to_vec();
    sorted.sort();

    Some(sorted.first().unwrap() + sorted.last().unwrap())
}
