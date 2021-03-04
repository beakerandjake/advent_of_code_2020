use std::error::Error;

type InstructionResult = (i32, i32);

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions: Vec<&str> = input.lines().collect();
    let mut visited = vec![false; instructions.len()];
    let mut accumulator = 0;
    let mut instruction_index = 0;

    while instruction_index < visited.len() && visited[instruction_index] == false {
        visited[instruction_index] = true;
        match run_instruction(instructions[instruction_index]) {
            None => return Err("could not parse instruction".into()),
            Some(result) => {
                accumulator += result.0;
                instruction_index = match result.1.is_positive() {
                    true => instruction_index + result.1 as usize,
                    false => instruction_index - result.1.abs() as usize,
                };
            }
        }
    }
    Ok(accumulator.to_string())
}

fn run_instruction(instruction: &str) -> Option<InstructionResult> {
    match &instruction[..3] {
        "jmp" => jmp(&instruction[4..]),
        "acc" => acc(&instruction[4..]),
        "nop" => Some((0, 1)),
        _ => None,
    }
}

fn jmp(argument: &str) -> Option<InstructionResult> {
    parse_signed_num(argument).and_then(|p| Some((0, p)))
}

fn acc(argument: &str) -> Option<InstructionResult> {
    parse_signed_num(argument).and_then(|p| Some((p, 1)))
}

fn parse_signed_num(input: &str) -> Option<i32> {
    input.chars().next().and_then(|sign| match sign {
        '+' => input[1..].parse().ok(),
        '-' => input.parse().ok(),
        _ => None,
    })
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}
