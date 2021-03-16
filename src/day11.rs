use std::error::Error;
use std::fmt;



#[derive(Debug)]
struct Layout {
    seats: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
}

impl Layout {
    const ADJACENCY_MATRIX: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn new(input: &str) -> Layout {
        let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let max_x = seats.len() as i32;
        let max_y = seats.first().map_or(0, |x| x.len()) as i32;

        Layout {
            seats,
            max_x,
            max_y,
        }
    }

    fn get_adjacent_seats(&self, x: i32, y: i32) {
        let z: Vec<char> = Layout::ADJACENCY_MATRIX
            .iter()
            .filter_map(|adj| self.get_seat(x + adj.0, y + adj.1))
            .collect();

        println!("{:?}", z);
    }

    fn get_seat(&self, x: i32, y: i32) -> Option<char> {
        if y < 0 || y > self.max_y || x < 0 || x > self.max_x {
            return None;
        }

        return Some(self.seats[y as usize][x as usize]);
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.seats {
            for seat in row {
                write!(f, "{}", seat);
            }
            writeln!(f);
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let layout = Layout::new(input);

    layout.get_adjacent_seats(7, 2);

    // let mut layout = Layout::new();

    // for line in input.lines() {
    //     let mut row = Vec::new();

    //     for character in line.chars() {
    //         if let Some(seat) = SeatType::parse(character) {
    //             row.push(seat);
    //         }
    //     }

    //     layout.push(row);
    // }

    //print_layout(&layout);

    Ok("great job".to_string())
}

// fn count_adjacent_seats(current: Position, layout: &Layout) {
//     let counts = AdjacentCounts {
//         Empty: 0,
//         Occupied: 0,
//     };

//     let z = ADJACENCY_MATRIX.iter().filter_map(|&adjacent| {
//         let adjacent_position = current.add(adjacent);
//         get_seat(adjacent_position, &layout)
//     });

//     unimplemented!();
// }

// fn get_seat<'a>(position: Position, layout: &'a Layout) -> Option<&'a SeatType> {
//     if position.y < 0 || position.y >= layout.len() as i32 {
//         return None;
//     }

//     let row = &layout[position.y as usize];

//     unimplemented!();
// }

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    Ok("great job".to_string())
}

// fn print_layout(layout: &Layout) {
//     for row in layout {
//         for seat in row {
//             print!("{}", seat);
//         }
//         println!();
//     }
// }
