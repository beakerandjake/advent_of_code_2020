use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug)]
struct Requirement<'a> {
    quantity: u32,
    color: &'a str,
}

impl<'a> Requirement<'a> {
    fn parse(input: &'a str) -> Option<Requirement> {
        lazy_static! {
            static ref PARSE_INPUT_REGEX: Regex = Regex::new(r"(\d+) (.*) bags?\.?$").unwrap();
        }

        // handle special case of no requirement.
        if input == "no other bags." {
            return None;
        }

        let (quantity, color) = match PARSE_INPUT_REGEX.captures(input) {
            Some(capture) => match (&capture[1]).parse::<u32>() {
                Ok(num) => (num, capture.get(2).unwrap().as_str()),
                Err(_e) => return None,
            },
            None => return None,
        };

        Some(Requirement { quantity, color })
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut bag_rules = HashMap::new();

    for line in input.lines() {
        let (bag, child_bags) = parse_part1_input_line(line)?;
        bag_rules.insert(bag, child_bags);
    }

    let mut target_count = 0;
    let target_color = "shiny gold";
    let mut known_holders = HashSet::new();

    for &bag_color in bag_rules.keys() {
        if bag_color != target_color
            && search_part1(bag_color, target_color, &bag_rules, &mut known_holders)
        {
            target_count = target_count + 1;
            known_holders.insert(bag_color);
        }
    }

    Ok(target_count.to_string())
}

fn parse_part1_input_line<'a>(input: &'a str) -> Result<(&'a str, Vec<&str>), Box<dyn Error>> {
    let mut parts = input.split(" bags contain ");
    let bag_name = parts.next().ok_or("could not get bag name.")?;

    lazy_static! {
        static ref PARSE_CHILD_REGEX: Regex = Regex::new(r"\d+ (.*) bags?\.?$").unwrap();
    }

    let child_bags: Vec<&str> = parts
        .next()
        .ok_or("could not get child bags")?
        .split(", ")
        .filter_map(|x| match PARSE_CHILD_REGEX.captures(x) {
            None => None,
            Some(captures) => Some(captures.get(1).unwrap().as_str()),
        })
        .collect();

    Ok((bag_name, child_bags))
}

fn search_part1<'a>(
    current_bag_color: &'a str,
    target_bag_color: &str,
    all_bags: &HashMap<&str, Vec<&'a str>>,
    known_holders: &mut HashSet<&'a str>,
) -> bool {
    if current_bag_color == target_bag_color {
        return true;
    }

    if known_holders.contains(current_bag_color) {
        return true;
    }

    let child_bags = match all_bags.get(current_bag_color) {
        None => return false,
        Some(x) => x,
    };

    for &current_color in child_bags {
        if search_part1(current_color, target_bag_color, all_bags, known_holders) {
            known_holders.insert(current_color);
            return true;
        }
    }

    return false;
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}