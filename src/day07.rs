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
        let (bag, child_bags) = parse_input_line(line)?;
        bag_rules.insert(bag, child_bags);
    }
    
    let mut target_count = 0;
    let target_color = "shiny gold";

    for &bag_color in bag_rules.keys() {
        if bag_color != target_color && search_child_bags(target_color, bag_color, &bag_rules) {
            target_count = target_count + 1;
        }
    }

    Ok(target_count.to_string())
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}

fn parse_input_line<'a>(input: &'a str) -> Result<(&'a str, Vec<Requirement>), Box<dyn Error>> {
    let mut parts = input.split(" bags contain ");
    let bag_name = parts.next().ok_or("could not get bag name.")?;
    let child_bags: Vec<Requirement> = parts
        .next()
        .ok_or("could not get child bags")?
        .split(", ")
        .filter_map(|x| Requirement::parse(x))
        .collect();

    Ok((bag_name, child_bags))
}

fn search_child_bags(target_color: &str, current_color: &str, bag_rules: &HashMap<&str, Vec<Requirement>>) -> bool {
    if target_color == current_color {
        return true;
    }
    
    let child_bags = match bag_rules.get(current_color) {
        None => return false,
        Some(x) => x
    };
    
    for child_bag in child_bags {
        if search_child_bags(target_color, child_bag.color, bag_rules) {
            return true;
        }
    }

    return false;
}
