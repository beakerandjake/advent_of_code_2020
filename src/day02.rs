use std::error::Error;

#[derive(Debug)]
struct Rule {
    min: i32,
    max: i32,
    letter: char,
}

impl Rule {
    pub fn new(rule_str: &str) -> Result<Self, Box<dyn Error>> {
        println!("parsing: {}", rule_str);
        let mut parts = rule_str.split_whitespace();

        let min_max = parts
            .next()
            .ok_or("Could not parse min/max from rule string")?
            .split('-');

        let letter = parts
            .next()
            .ok_or("Could not parse letter from rule string")?;

        println!("min/max:{} letter:{}", min_max, letter);

        Ok(Rule {
            min: 1,
            max: 2,
            letter: 'a',
        })
    }

    fn parse_min_max(input:&str) -> Result<(i32,i32), Box<dyn Error>> {
        // 13-14
        let parts: Vec<i32> = input
            .split('-')
            .map(|x| x.parse::<i32>());

        unimplemented!();
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let line = input.lines().next().unwrap();

    let result = parse_rule(line)?;

    println!("rule: {:?} password: {}", result.0, result.1);

    Ok(String::from("unimplemented"))
}

fn parse_rule(input: &str) -> Result<(Rule, &str), Box<dyn Error>> {
    let mut parts = input.split(':');

    let rule_str = parts
        .next()
        .ok_or("Could not get rule portion of input string")?;

    let password = parts
        .next()
        .ok_or("Could not get password portion of input string")?;

    Ok((Rule::new(rule_str)?, password)) 
}
