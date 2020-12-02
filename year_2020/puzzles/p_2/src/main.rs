use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut num_valid: i32 = 0;
    let mut num_invalid: i32 = 0;
    if let Ok(lines) = read_lines("puzzles/p_2/data/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                // do work
                if let Ok(parts) = get_parts(v.as_str()) {
                    let (min_num, max_num, char_match, password) = parts;

                    /* part 1
                    let found: Vec<_> = password.match_indices(char_match.as_str()).collect();
                    let num_chars_found = found.len();
                    if num_chars_found < min_num || num_chars_found > max_num {
                        println!("FAIL [line=\"{}\"] min={} max={} char_match={} password={}", v, min_num, max_num, char_match, password);
                        num_invalid += 1;
                    } else {
                        println!("PASS [line=\"{}\"] min={} max={} char_match={} password={}", v, min_num, max_num, char_match, password);
                        num_valid += 1;
                    }
                    */

                    /* part 2 */
                    let idx_1 = min_num - 1;
                    let idx_2 = max_num - 1;
                    let found: Vec<_> = password.match_indices(char_match.as_str()).filter(
                        |m| m.0 == idx_1 || m.0 == idx_2
                    ).collect();
                    if found.len() != 1  {
                        println!("FAIL [line=\"{}\"] min={} max={} char_match={} password={}", v, min_num, max_num, char_match, password);
                        num_invalid += 1;
                    } else {
                        println!("PASS [line=\"{}\"] min={} max={} char_match={} password={} found={:?}", v, min_num, max_num, char_match, password, found);
                        num_valid += 1;
                    }
                }
            }
        }
    }
    println!("num_valid={} num_invalid={}", num_valid, num_invalid);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_parts(line: &str) -> Result<(usize, usize, String, String), io::Error> {
    // example: "4-7 l: lsplglldx"
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() == 2 {
        let password_def: Vec<&str> = parts[0].split(|c| c == ' ' || c == '-').collect();
        if password_def.len() == 3 {
            let min_num = password_def[0].parse::<usize>().unwrap();
            let max_num = password_def[1].parse::<usize>().unwrap();
            let char_min = password_def[2];
            return Ok((min_num, max_num, char_min.to_string(), parts[1].trim_start().to_string()))
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "invalid parts"))
}
