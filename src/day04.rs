use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut required_keys = HashSet::new();
    required_keys.insert("byr");
    required_keys.insert("iyr");
    required_keys.insert("eyr");
    required_keys.insert("hgt");
    required_keys.insert("hcl");
    required_keys.insert("ecl");
    required_keys.insert("pid");

    let valid_passport_count = parse_passports(input)
        .filter(|passport| {
            passport
                .keys()
                .filter(|key| required_keys.contains(*key))
                .count()
                >= required_keys.len()
        })
        .count();

    Ok(valid_passport_count.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let hcl_regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^[\d]{9}$").unwrap();
    let hgt_regex = Regex::new(r"^(\d{1,3})(cm|in)$").unwrap();
    let valid_eye_colors: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();

    // map a passport field to a validation function that knows how to validate that fields value.
    let mut validators: HashMap<&str, Box<dyn Fn(&str) -> bool>> = HashMap::new();
    validators.insert("byr", Box::new(|x| validate_numeric_range(x, 1920, 2002)));
    validators.insert("iyr", Box::new(|x| validate_numeric_range(x, 2010, 2020)));
    validators.insert("eyr", Box::new(|x| validate_numeric_range(x, 2020, 2030)));
    validators.insert("hcl", Box::new(|x| hcl_regex.is_match(x)));
    validators.insert("ecl", Box::new(|x| valid_eye_colors.contains(x)));
    validators.insert("pid", Box::new(|x| pid_regex.is_match(x)));
    validators.insert("hgt", Box::new(|x| validate_height(x, &hgt_regex)));

    let valid_passport_count = parse_passports(input)
        .filter(|passport| {
            validators
                .iter()
                .all(|(key, validator_fn)| match passport.get(key) {
                    None => false,
                    Some(value) => validator_fn(value),
                })
        })
        .count();

    Ok(valid_passport_count.to_string())
}

// returns an iterator which iterates each passport entry of the input
// and returns a HashMap of Key/Values contained in the passport.
fn parse_passports<'a>(input: &'a str) -> Box<dyn Iterator<Item = HashMap<&'a str, &'a str>> + 'a> {
    Box::new(input.split("\r\n\r\n").map(|entry| {
        entry
            .split(|c| c == ' ' || c == '\r' || c == '\n')
            .filter_map(|kvp| {
                let mut result = kvp.split(':');
                let key = result.next();
                let value = result.next();
                key.zip(value)
            })
            .collect()
    }))
}

fn validate_numeric_range(value: &str, min: i32, max: i32) -> bool {
    match value.parse::<i32>() {
        Ok(parsed) => parsed >= min && parsed <= max,
        Err(_e) => false,
    }
}

fn validate_height(value: &str, regex: &Regex) -> bool {
    match regex.captures(value) {
        None => false,
        Some(ref capture) => match &capture[2] {
            "cm" => validate_numeric_range(&capture[1], 150, 193),
            "in" => validate_numeric_range(&capture[1], 59, 76),
            _ => false,
        },
    }
}
