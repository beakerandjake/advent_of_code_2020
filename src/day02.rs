use std::error::Error;

// rule struct contains all the information about a password rule.
#[derive(Debug)]
struct Rule {
    min: i32,
    max: i32,
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
        let (min,max) = Rule::parse_min_max(&min_max_string)?;

        // expect second part of rule string to b e the letter portion.
        let letter_string = parts.next().ok_or("missing letter portion of rule")?;

        // grab only the first character.
        let letter = letter_string.chars().next().ok_or("letter not specified")?;
      
        Ok(Rule { min, max, letter: letter })
    }

    // parses a min/max string in the format 'X-Y' and returns the parsed min/max values.
    fn parse_min_max(input: &str) -> Result<(i32,i32), Box<dyn Error>> {
        let mut parts = input.split('-');

        // expect min as first number.
        let min: i32 = parts
            .next()
            .ok_or("min is missing")?
            .parse()
            .or(Err("min was not numeric"))?;

        // expect max as second number
        let max: i32 = parts
            .next()
            .ok_or("max is missing")?
            .parse()
            .or(Err("max was not numeric"))?;

        Ok((min,max))
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {

    let mut valid_password_count = 0;

    for line in input.lines() {
        let (rule, password) = parse_rule_and_password(&line)?;

        if validate_password(&password, &rule) {
            valid_password_count += 1;
        }
    }

    Ok(format!("Valid Passwords: {}", valid_password_count))
}

// parse an input string in the format 'X-Y Z: P'
// were 'X-Y Z' is the rule string and 'P' is the password. 
fn parse_rule_and_password(input: &str) -> Result<(Rule, &str), Box<dyn Error>> {
    let mut parts = input.split(':');

    let rule_str = parts
        .next()
        .ok_or("Could not get rule portion of input string")?;

    let password = parts
        .next()
        .ok_or("Could not get password portion of input string")?;

    Ok((Rule::new(&rule_str)?, password)) 
}

fn validate_password(password: &str, rule: &Rule) -> bool {
    false
}
