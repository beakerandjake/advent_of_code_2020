use std::collections::HashSet;
use std::error::Error;
use std::ops::Add;

const TREE_INPUT_CHAR: char = '#';

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Map {
    pattern_max_x: u32,
    pattern_max_y: u32,
    tree_coordinates: HashSet<Point>,
}

impl Map {
    fn new(map_input: &str) -> Map {
        let mut max_y: u32 = 0;
        let mut max_x: u32 = 0;
        let mut tree_coordinates = HashSet::new();

        for line in map_input.lines() {
            for (char_index, character) in line.chars().enumerate() {
                // only care about map coordinates which are for trees.
                // there isn't a point of storing anything if there is no terrain there.
                if character == TREE_INPUT_CHAR {
                    tree_coordinates.insert(Point {
                        x: char_index as u32,
                        y: max_y,
                    });
                }
            }

            // expect all lines have same number of chars.
            // set max x based on first row.
            if max_y == 0 {
                max_x = line.chars().count() as u32 - 1;
            }

            max_y += 1;
        }

        Map {
            pattern_max_x: max_x,
            pattern_max_y: max_y,
            tree_coordinates,
        }
    }

    fn sled_to_bottom(&self, starting_point: Point, slope: Point) -> u32 {
        let mut current_position = starting_point;
        let mut tree_count = 0;

        while current_position.y < self.pattern_max_y {
            current_position = current_position + slope;

            if self.is_tree(current_position) {
                tree_count += 1;
            }
        }

        tree_count
    }

    fn is_tree(&self, point: Point) -> bool {
        let translated_point = Point {
            x: point.x % (self.pattern_max_x + 1),
            y: point.y,
        };

        self.tree_coordinates.contains(&translated_point)
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let map = Map::new(&input);
    let slope = Point { x: 3, y: 1 };
    let tobaggan_position = Point { x: 0, y: 0 };
    
    let tree_count = map.sled_to_bottom(tobaggan_position, slope);

    Ok(format!("hit: {} trees.",tree_count))
}
