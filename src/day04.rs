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
        .filter(|passport| passport
            .keys()
            .filter(|key| required_keys.contains(*key))
            .count() >= required_keys.len()
        )
        .count();

    Ok(valid_passport_count.to_string())
}

// returns an iterator which iterates each passport entry of the input
// and returns a HashMap of Key/Values contained in the passport.
fn parse_passports<'a>(input: &'a str) ->  Box<dyn Iterator<Item = HashMap<&'a str, &'a str>> + 'a> {
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