use std::error::Error;

type InstructionResult = (i32, i32);

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions: Vec<&str> = input.lines().collect();
    let mut visited = vec![false; instructions.len()];
    let mut accumulator = 0;
    let mut instruction_index = 0;

    while instruction_index < visited.len() && visited[instruction_index] == false {
        visited[instruction_index] = true;
        let instruction = instructions[instruction_index];
        match run_instruction(&instruction[..3], &instruction[4..]) {
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

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let instructions: Vec<&str> = input.lines().collect();

    for (index, instruction) in instructions.iter().enumerate() {
        let argument = &instruction[4..];
        let name = match replace_instruction(&instruction[..3]) {
            None => continue,
            Some(x) => x,
        };

        if let Some(acc) = test_replacement(name, argument, index, &instructions) {
            return Ok(acc.to_string());
        }
    }

    Err("could not find instruction".into())
}

fn replace_instruction<'a>(instruction: &'a str) -> Option<&'a str> {
    match instruction {
        "nop" => Some("jmp"),
        "jmp" => Some("nop"),
        _ => None,
    }
}

fn test_replacement(
    replacement_instruction: &str,
    replacement_argumnet: &str,
    replacement_index: usize,
    instructions: &Vec<&str>,
) -> Option<i32> {
    let mut index = 0;
    let mut accumulator = 0;
    let mut visited = vec![false; instructions.len()];

    while index < instructions.len() {
        if visited[index] == true {
            return None;
        }
        visited[index] = true;

        let (instruction, argument) = if index == replacement_index {
            (replacement_instruction, replacement_argumnet)
        } else {
            (&instructions[index][..3], &instructions[index][4..])
        };

        if let Some(result) = run_instruction(instruction, argument) {
            accumulator += result.0;
            index = match result.1.is_positive() {
                true => index + result.1 as usize,
                false => index - result.1.abs() as usize,
            };
        }
    }
    Some(accumulator)
}

fn run_instruction(instruction: &str, argument: &str) -> Option<InstructionResult> {
    match instruction {
        "jmp" => jmp(argument),
        "acc" => acc(argument),
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
