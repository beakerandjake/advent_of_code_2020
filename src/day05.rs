use std::cmp::Ordering;
use std::error::Error;
// use math::round;

enum SearchHalf {
    Upper,
    Lower,
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let rows: Vec<u32> = (0..128).collect();
    let seats: Vec<u32> = (0..8).collect();

    let mut highest_seat_id = 0;

    for boarding_pass in input.lines() {
        if boarding_pass.len() != 10 {
            Err("Invalid Boarding Pass!")?
        }

        let row_number = follow_directions(&rows, &boarding_pass[..7])?;
        let seat_number = follow_directions(&seats, &boarding_pass[7..])?;
        let seat_id = row_number * 8 + seat_number;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    Ok(highest_seat_id.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok("Great Job".to_string())
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