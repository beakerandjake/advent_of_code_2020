use std::error::Error;

enum TerrainType {
    Tree,
    Empty,
}

#[derive(Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut tree_coordinates = Vec::new();

    for (y_index, line) in input.lines().enumerate() {
        for (x_index, character) in line.chars().enumerate() {
            if let TerrainType::Tree = get_terrain_type(character) {
                tree_coordinates.push(Coordinate {
                    x: x_index as u32,
                    y: y_index as u32,
                });
            }

            if x_index > max_x {
                max_x = x_index
            }
        }

        max_y += 1;
    }

    println!("max_x = {}, max_y = {}, tree_coordinates: {:?}", max_x, max_y, tree_coordinates);

    Ok("Great!".to_string())
}

fn get_terrain_type(character: char) -> TerrainType {
    match character {
        '#' => TerrainType::Tree,
        _ => TerrainType::Empty,
    }
}
