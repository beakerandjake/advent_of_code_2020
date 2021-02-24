use std::collections::HashSet;
use std::error::Error;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let rows: Vec<u32> = (0..128).collect();
    let seats: Vec<u32> = (0..8).collect();

    let mut highest_seat_id = 0;

    for boarding_pass in input.lines() {
        let seat_id = get_seat_id(&rows, &seats, boarding_pass)?;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    Ok(highest_seat_id.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let rows: Vec<u32> = (0..128).collect();
    let seats: Vec<u32> = (0..8).collect();

    let seat_ids: HashSet<u32> = input
        .lines()
        .filter_map(|boarding_pass| get_seat_id(&rows, &seats, boarding_pass).ok())
        .collect();

    for row in rows {
        for seat in &seats {
            let seat_id = calculate_seat_id(row, *seat);

            if !seat_ids.contains(&seat_id)
                && seat_ids.contains(&(seat_id + 1))
                && seat_ids.contains(&(seat_id - 1))
            {
                return Ok(seat_id.to_string());
            }
        }
    }

    Ok("not found".to_string())
}

fn get_seat_id(rows: &[u32], seats: &[u32], boarding_pass: &str) -> Result<u32, &'static str> {
    if boarding_pass.len() != 10 {
        Err("Invalid Boarding Pass!")?
    }

    let row_number = follow_directions(&rows, &boarding_pass[..7])?;
    let seat_number = follow_directions(&seats, &boarding_pass[7..])?;

    Ok(calculate_seat_id(*row_number, *seat_number))
}

fn follow_directions<'a>(items: &'a [u32], directions: &str) -> Result<&'a u32, &'static str> {
    let mut slice = items;

    for direction in directions.chars() {
        let m = slice.len() / 2;

        slice = match direction {
            // upper half
            'B' | 'R' => &slice[m..slice.len()],
            // lower half
            'F' | 'L' => &slice[0..m],
            _ => Err("Unknown direction in input")?,
        };
    }

    slice.first().ok_or("could not follow directions")
}

fn calculate_seat_id(row_number: u32, seat_number: u32) -> u32 {
    row_number * 8 + seat_number
}
