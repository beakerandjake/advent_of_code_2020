use std::error::Error;

const MIN_MAX_SEPARATOR: char = '-';
const RULE_AND_PASSWORD_SEPARATOR: char = ':';

// represents a fn which tests a rule against a password. 
type RuleValidatorFn = fn(&str, &Rule) -> bool;

// rule struct contains all the information about a password rule.
#[derive(Debug)]
struct Rule {
    min: u32,
    max: u32,
    letter: char,
}

impl Rule {
    // rule string expected format is "X-Y Z" where x is min, y is max and z is the letter.
    pub fn new(rule_str: &str) -> Result<Rule, Box<dyn Error>> {
        // split on whitespace so we get ['X-Y', 'Z']
        let mut parts = rule_str.split_whitespace();

        // expect first part of rule string to be the min/max string 'X-Y'
        let min_max_string = parts.next().ok_or("missing min/max portion of rule")?;

        // parse the min/max numbers from the string.
        let (min, max) = Rule::parse_min_max(&min_max_string)?;

        // expect second part of rule string to b e the letter portion.
        let letter_string = parts.next().ok_or("missing letter portion of rule")?;

        // grab only the first character.
        let letter = letter_string.chars().next().ok_or("letter not specified")?;

        Ok(Rule {
            min,
            max,
            letter: letter,
        })
    }

    // parses a min/max string in the format 'X-Y' and returns the parsed min/max values.
    fn parse_min_max(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
        let mut parts = input.split(MIN_MAX_SEPARATOR);

        // expect min as first number.
        let min: u32 = parts
            .next()
            .ok_or("min is missing")?
            .parse()
            .or(Err("couldn't parse min"))?;

        // expect max as second number
        let max: u32 = parts
            .next()
            .ok_or("max is missing")?
            .parse()
            .or(Err("couldn't parse max"))?;

        Ok((min, max))
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let valid_password_count = validate_passwords(input, part1_validator)?;
    Ok(format!("Valid Passwords: {}", valid_password_count))
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let valid_password_count = validate_passwords(input, part2_validator)?;
    Ok(format!("Valid Passwords: {}", valid_password_count))
}

// iterate every line of input, parsing a rule and a password, then validating the password
// with the validator fn.
fn validate_passwords(input: &str, validation_fn: RuleValidatorFn) -> Result<u32, Box<dyn Error>> {
    let mut valid_password_count = 0;

    for line in input.lines() {
        let (rule, password) = parse_rule_and_password(&line)?;

        if validation_fn(&password, &rule) {
            valid_password_count += 1;
        }
    }

    Ok(valid_password_count)
}

// parse an input string in the format 'X-Y Z: P'
// were 'X-Y Z' is the rule string and 'P' is the password.
fn parse_rule_and_password(input: &str) -> Result<(Rule, &str), Box<dyn Error>> {
    let mut parts = input.split(RULE_AND_PASSWORD_SEPARATOR);

    let rule_str = parts
        .next()
        .ok_or("Could not get rule portion of input string")?;

    let password = parts
        .next()
        .ok_or("Could not get password portion of input string")?
        .trim();

    Ok((Rule::new(&rule_str)?, password))
}


// returns true if the password satisfied the criteria for part1
fn part1_validator(password: &str, rule: &Rule) -> bool {
    let char_count = password.chars().filter(|c| *c == rule.letter).count() as u32;

    char_count >= rule.min && char_count <= rule.max
}

// returns true if the password satisfied the criteria for part2
fn part2_validator(password: &str, rule: &Rule) -> bool {
    let mut match_1 = false;
    let mut match_2 = false;

    for (index, character) in password.chars().enumerate() {
        let password_index = (index as u32) + 1;
        
        if rule.min == password_index {
            if character == rule.letter {
                match_1 = true;
            }
        }
        if rule.max == password_index {
            if character == rule.letter {
                match_2 = true;
            }
        }
    }

    match_1 != match_2
}