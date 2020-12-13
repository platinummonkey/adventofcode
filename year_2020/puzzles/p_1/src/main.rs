use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut all_values: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("puzzles/p_1/data/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                all_values.push(v.parse::<i32>().unwrap());
            }
        }
    }

    for i in 0..all_values.len() - 1 {
        for j in i + 1..all_values.len() {
            for k in j + 1..all_values.len() {
                if all_values[i] + all_values[j] + all_values[k] == 2020 {
                    let val = all_values[i] * all_values[j] * all_values[k];
                    println!(
                        "found pair = {}(i={}) and {}(j={}) and {}(k={}) value={}",
                        all_values[i], i, all_values[j], j, all_values[k], k, val
                    );
                    return;
                }
            }
        }
    }
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
