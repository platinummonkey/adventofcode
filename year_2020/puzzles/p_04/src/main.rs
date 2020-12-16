use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Passport {
    data: HashMap<String, String>,
    valid: bool,
}

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    fn new(line: String) -> Passport {
        let mut p = Passport {
            data: HashMap::new(),
            valid: false,
        };
        for part in line.split_ascii_whitespace() {
            let key_value: Vec<_> = part.splitn(2, ":").collect();
            if key_value.len() == 2 {
                let v = key_value[1].to_string();
                let valid_value = match key_value[0] {
                    "byr" => valid_int_range(v, 1920, 2002),
                    "iyr" => valid_int_range(v, 2010, 2020),
                    "eyr" => valid_int_range(v, 2020, 2030),
                    "hgt" => valid_height(v),
                    "hcl" => valid_hair_color(v),
                    "ecl" => valid_eye_color(v),
                    "pid" => valid_passport_id(v),
                    "cid" => Result::Ok(v), // optional safe to ignore
                    _ => Err(String::from("discarded")), // discard
                };
                match valid_value {
                    Ok(valid_key_value) => p.data.insert(key_value[0].to_string(), valid_key_value),
                    Err(_e) => None, // discard
                };
            }
        }
        // now validate all required fields were set
        p.valid = REQUIRED_KEYS.iter().all(|k| p.data.contains_key(*k));
        p
    }
}

fn valid_height(value: String) -> Result<String, String> {
    let re = Regex::new(r"(\d+)([incm]{2})").unwrap();
    let invalid = Err(String::from("invalid hair color"));
    let m = re.captures_iter(value.as_str()).next();
    match m {
        None => invalid,
        Some(cap) => {
            let num_str = cap[1].to_string().to_owned();
            let unit = &cap[2];
            match unit {
                "cm" => match valid_int_range(num_str, 150, 193) {
                    Ok(_) => Ok(value),
                    Err(e) => Err(e),
                },
                "in" => match valid_int_range(num_str, 59, 76) {
                    Ok(_) => Ok(value),
                    Err(e) => Err(e),
                },
                _ => invalid,
            }
        }
    }
}

fn valid_hair_color(value: String) -> Result<String, String> {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    match re.is_match(value.as_str()) {
        true => Ok(value.to_string()),
        false => Err(String::from("invalid hair color")),
    }
}

fn valid_passport_id(value: String) -> Result<String, String> {
    let re = Regex::new(r"^\d{9}$").unwrap();
    match re.is_match(value.as_str()) {
        true => Ok(value.to_string()),
        false => Err(String::from("invalid passport ID")),
    }
}

fn valid_eye_color(value: String) -> Result<String, String> {
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    match re.is_match(value.as_str()) {
        true => Ok(value.to_string()),
        false => Err(String::from("invalid passport ID")),
    }
}

fn valid_int_range(value: String, min_value: i32, max_value: i32) -> Result<String, String> {
    let result = value.parse::<i32>();
    let r: Result<String, String> = match result {
        Ok(i) => {
            if i >= min_value && i <= max_value {
                Ok(value)
            } else {
                Err(String::from("not a valid byr must be between [1920, 2002]"))
            }
        }
        Err(_e) => Err(String::from("not valid byr")),
    };
    r
}

fn main() {
    // File must exist in current path before this produces output
    let mut num_valid_passports: i32 = 0;

    if let Ok(lines) = read_lines("puzzles/p_04/data/input") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_passport: String = String::new();
        for line in lines {
            if let Ok(v) = line {
                // do work
                if v.len() != 0 {
                    current_passport.push(' ');
                    current_passport.push_str(v.as_str());
                } else {
                    let p = Passport::new(current_passport.clone());
                    if p.valid {
                        // part 1
                        num_valid_passports += 1;
                    }
                    current_passport.clear()
                }
            }
        }
        // final line edge case
        if current_passport.len() > 0 {
            let p = Passport::new(current_passport.clone());
            if p.valid {
                // part 1
                num_valid_passports += 1;
            }
        }
    }
    println!("num valid passports: {}", num_valid_passports)
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
