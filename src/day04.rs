use std::collections::HashSet;
use std::error::Error;

const NEW_LINE: &str = "\r\n";
const ENTRY_SEPARATOR: &str = "\r\n\r\n";

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut required_keys = HashSet::new();
    required_keys.insert("byr");
    required_keys.insert("iyr");
    required_keys.insert("eyr");
    required_keys.insert("hgt");
    required_keys.insert("hcl");
    required_keys.insert("ecl");
    required_keys.insert("pid");

    let mut valid_passport_count = 0;

    for entry in input.split(ENTRY_SEPARATOR) {
        let sanitized = entry.replace(NEW_LINE, " ");
        let key_value_pairs: Vec<&str> = sanitized.split(" ").collect();
        
        // passport can only be valid if it has at least the min number of fields.
        if key_value_pairs.len() < 7 {
            continue;
        }

        // get the total number of keys in the passport which were required.
        let number_of_required_fields = key_value_pairs
            .iter()
            .filter_map(|kvp| kvp.split(':').next())
            .collect::<HashSet<&str>>()
            .intersection(&required_keys)
            .count();

        if number_of_required_fields >= required_keys.len() {
            valid_passport_count += 1;
        }
    }

    Ok(valid_passport_count.to_string())
}
