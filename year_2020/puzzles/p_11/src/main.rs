use crate::Location::{EmptySeat, Floor, OccupiedSeat};
#[allow(unused_imports)]
use util::*;

const DEBUG_WITH_IMAGES: bool = true;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_11/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_11/data/input"));
}

fn num_adjacent(seats: Vec<Vec<Location>>, row: i32, col: i32) -> i32 {
    let mut num_adjacent: i32 = 0;

    let neighbors = VISIBLE_VECTORS
        .clone()
        .iter()
        .map(|(r, c)| ((row + r), (col + c)))
        .collect::<Vec<(i32, i32)>>();

    for (r, c) in neighbors {
        if r < 0 || c < 0 {
            continue;
        }
        match seats.get(r as usize) {
            Some(seat_row) => match seat_row.get(c as usize) {
                Some(l) => match l {
                    OccupiedSeat => num_adjacent += 1,
                    _ => {}
                },
                None => {}
            },
            None => {}
        }
    }
    num_adjacent
}

fn apply_rules_1(seats: Vec<Vec<Location>>) -> (Vec<Vec<Location>>, bool, i64) {
    let rows = seats.len();
    let cols = seats[0].len();
    let mut new_seats: Vec<Vec<Location>> = Vec::new();
    let mut modified: bool = false;
    let mut num_occupied: i64 = 0;
    for row in 0..rows {
        let mut new_cols: Vec<Location> = Vec::new();
        for col in 0..cols {
            let current_location = seats[row][col].clone();
            let num_occupied_near = num_adjacent(seats.clone(), row as i32, col as i32);
            let new_location = match current_location {
                EmptySeat => {
                    // if there are no occupied seats adjacent, occupy it
                    match num_occupied_near {
                        0 => {
                            modified = true;
                            num_occupied += 1;
                            OccupiedSeat
                        }
                        _ => EmptySeat,
                    }
                }
                OccupiedSeat => {
                    // if there are four or more seats adjacent also occupied then go empty
                    match num_occupied_near {
                        0..=3 => {
                            num_occupied += 1;
                            OccupiedSeat
                        }
                        _ => {
                            modified = true;
                            EmptySeat
                        }
                    }
                }
                Floor => Floor,
            };
            new_cols.push(new_location);
        }
        new_seats.push(new_cols);
    }
    (new_seats, modified, num_occupied)
}

fn generate_image(state: Vec<Vec<Location>>) {
    if !DEBUG_WITH_IMAGES {
        return;
    }
    print!("\n");
    for row in state {
        for col in row {
            match col {
                OccupiedSeat => print!("#"),
                EmptySeat => print!("L"),
                _ => print!("."),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn part_1(filename: &str) -> i64 {
    let mut seats = read_raw_data(filename);
    let mut num_loops = 0;
    let mut num_occupied: i64 = 0;
    loop {
        generate_image(seats.clone());
        let (new_seats, modified, new_num_occupied) = apply_rules_1(seats.clone());
        if modified {
            num_occupied = new_num_occupied;
            seats = new_seats.clone();
        } else {
            break;
        }
        num_loops += 1;
    }
    generate_image(seats.clone());
    num_occupied
}

const VISIBLE_VECTORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn num_visible(seats: Vec<Vec<Location>>, row: i32, col: i32) -> i32 {
    let mut num_visible: i32 = 0;
    let max_row = (seats.len() - 1) as i32;
    let max_col = (seats[0].len() - 1) as i32;

    for vector in VISIBLE_VECTORS.iter() {
        let mut current_row = row.clone();
        let mut current_col = col.clone();
        'vec_search: loop {
            current_row += vector.0;
            current_col += vector.1;
            if current_row < 0 || current_col < 0 || current_row > max_row || current_col > max_col
            {
                break 'vec_search;
            }
            match seats.get(current_row as usize) {
                Some(seat_row) => match seat_row.get(current_col as usize) {
                    Some(l) => match l {
                        OccupiedSeat => {
                            num_visible += 1;
                            break 'vec_search;
                        }
                        EmptySeat => {
                            // first visible, just break
                            break 'vec_search;
                        }
                        _ => {}
                    },
                    None => {}
                },
                None => {}
            }
        }
    }

    num_visible
}

fn apply_rules_2(seats: Vec<Vec<Location>>) -> (Vec<Vec<Location>>, bool, i64) {
    let rows = seats.len();
    let cols = seats[0].len();
    let mut new_seats: Vec<Vec<Location>> = Vec::new();
    let mut modified: bool = false;
    let mut num_occupied: i64 = 0;
    for row in 0..rows {
        let mut new_cols: Vec<Location> = Vec::new();
        for col in 0..cols {
            let current_location = seats[row][col].clone();
            let num_occupied_visible = num_visible(seats.clone(), row as i32, col as i32);
            let new_location = match current_location {
                EmptySeat => {
                    // if there are no occupied seats adjacent, occupy it
                    match num_occupied_visible {
                        0 => {
                            modified = true;
                            num_occupied += 1;
                            OccupiedSeat
                        }
                        _ => EmptySeat,
                    }
                }
                OccupiedSeat => {
                    // if there are four or more seats adjacent also occupied then go empty
                    match num_occupied_visible {
                        0..=4 => {
                            num_occupied += 1;
                            OccupiedSeat
                        }
                        _ => {
                            modified = true;
                            EmptySeat
                        }
                    }
                }
                Floor => Floor,
            };
            new_cols.push(new_location);
        }
        new_seats.push(new_cols);
    }
    (new_seats, modified, num_occupied)
}

fn part_2(filename: &str) -> i64 {
    let mut seats = read_raw_data(filename);
    let mut num_loops = 0;
    let mut num_occupied: i64 = 0;
    loop {
        generate_image(seats.clone());
        let (new_seats, modified, new_num_occupied) = apply_rules_2(seats.clone());
        if modified {
            num_occupied = new_num_occupied;
            seats = new_seats.clone();
        } else {
            break;
        }
        num_loops += 1;
    }
    generate_image(seats.clone());
    num_occupied
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Location {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn read_raw_data(filename: &str) -> Vec<Vec<Location>> {
    let mut seats: Vec<Vec<Location>> = Vec::new();
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut col_seats: Vec<Location> = Vec::new();
            for c in line.chars() {
                let l = match c {
                    '#' => OccupiedSeat,
                    'L' => EmptySeat,
                    _ => Floor,
                };
                col_seats.push(l);
            }
            seats.push(col_seats);
        }
    }
    seats
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 2254;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 2004;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
