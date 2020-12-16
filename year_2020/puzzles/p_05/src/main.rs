use std::cell::Cell;
use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_ROW: i32 = 127;
const MAX_COL: i32 = 7;

#[derive(Debug, Clone, Copy)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
    used: bool,
}

impl Seat {
    fn new(row: i32, col: i32) -> Seat {
        Seat {
            row: row,
            col: col,
            id: row * 8 + col,
            used: false,
        }
    }
}

struct Seats {
    seats: Vec<Vec<Cell<Seat>>>,
}

impl Seats {
    fn new(max_row: i32, max_col: i32) -> Seats {
        let mut s = Seats { seats: Vec::new() };
        for row in 0..max_row + 1 {
            s.seats.push(Vec::new());
            for col in 0..max_col + 1 {
                s.seats[row as usize].push(Cell::from(Seat::new(row, col)))
            }
        }
        s
    }

    fn set_used(&mut self, row: i32, col: i32) {
        self.seats[row as usize][col as usize].get_mut().used = true
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for fun visual display
        let mut buf: String = String::new();
        for row in self.seats.iter() {
            for seat in row.iter() {
                let seat_id = seat.get().id;
                match seat.get().used {
                    true => buf.push_str(format!("[{:>4}:X] ", seat_id).as_str()),
                    false => buf.push_str(format!("[{:>4}: ] ", seat_id).as_str()),
                }
            }
            buf.push_str("\n")
        }
        write!(f, "Seats:\n{}", buf)
    }
}

fn main() {
    // File must exist in current path before this produces output
    let mut max_seat_id = 0;
    let mut seat_ids: Vec<i32> = Vec::new();
    let mut seats: Seats = Seats::new(MAX_ROW, MAX_COL);

    if let Ok(lines) = read_lines("puzzles/p_05/data/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                let (seat_row, seat_col, seat_id) = bin_seat(v.clone());
                seat_ids.push(seat_id);
                seats.set_used(seat_row, seat_col);
                max_seat_id = max(seat_id, max_seat_id);
                // println!("{} = row={} col={} id={}", v, seat_row, seat_col, seat_id)
            }
        }
    }
    seat_ids.sort();
    println!("max seat id is {}", max_seat_id);
    // lets visualize for fun
    println!("seats = {}", seats);

    let mut idx = 1;
    while idx + 1 < seat_ids.len() {
        if seat_ids[idx] + 1 != seat_ids[idx + 1] {
            println!("seat id is {}", seat_ids[idx] + 1)
        }

        idx += 1;
    }
}

fn bin_seat(line: String) -> (i32, i32, i32) {
    let mut idx = 0;
    let mut seat_row: i32 = 0;
    let mut seat_col: i32 = 0;
    let mut seat_id: i32 = 0;
    for c in line.chars() {
        seat_id *= 2;
        match c {
            'B' | 'R' => seat_id += 1,
            _ => {}
        }
        match idx {
            0..=6 => {
                seat_row *= 2;
                match c {
                    'B' => seat_row += 1,
                    _ => {}
                }
            }
            _ => {
                seat_col *= 2;
                match c {
                    'R' => seat_col += 1,
                    _ => {}
                }
            }
        }

        idx += 1;
    }
    (seat_row, seat_col, seat_id)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
