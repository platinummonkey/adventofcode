#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_14/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_14/data/input"));
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Mask {
        force_on: Vec<u8>,
        floating: Vec<u8>,
        force_off: Vec<u8>,
    },
    Set {
        location: u16,
        value: u64,
    },
}

impl From<String> for Operation {
    fn from(line: String) -> Self {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        return match parts[0] {
            "mask" => {
                // todo
                let str_mask = parts.last().expect("expected value").to_string();
                let mut force_on: Vec<u8> = Vec::new();
                let mut force_off: Vec<u8> = Vec::new();
                let mut floating: Vec<u8> = Vec::new();
                for (i, v) in str_mask.chars().rev().enumerate() {
                    match v {
                        '1' => force_on.push(i as u8),
                        '0' => force_off.push(i as u8),
                        'X' => floating.push(i as u8),
                        _ => {} // skip
                    }
                }
                Operation::Mask {
                    force_on,
                    floating,
                    force_off,
                }
            }
            _ => {
                // todo
                let location = parts[0]
                    .trim_end_matches(']')
                    .trim_start_matches("mem[")
                    .parse::<u16>()
                    .expect("expected valid integer memory location");
                let value = parts
                    .last()
                    .expect("expected value")
                    .parse::<u64>()
                    .expect("expected valid memory allocation value");
                Operation::Set { location, value }
            }
        };
    }
}

#[allow(unused_variables)]
impl Operation {
    fn apply_mask(&self, val: u64) -> u64 {
        match self {
            Operation::Mask {
                force_on,
                floating,
                force_off,
            } => {
                let mut new_val = val.clone();
                // set all required bits
                for &bit in force_on {
                    new_val |= 1 << bit;
                }
                for &bit in force_off {
                    let mask = 1 << bit;
                    new_val = (new_val & !mask) | ((0 << bit) & mask);
                }
                return new_val;
            }
            _ => panic!("imposta!"),
        }
    }

    fn apply_mask_v2(&self, val: u64) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        match self {
            Operation::Mask {
                force_on,
                floating,
                force_off,
            } => {
                let floating_combination_iter = floating
                    .iter()
                    .map(|&i| Vec::from([(i, 0), (i, 1)]))
                    .flatten()
                    .combinations(floating.len())
                    .filter(|c| c.iter().map(|v| v.0).unique().count() == floating.len());
                for combination in floating_combination_iter {
                    let mut new_force_on = force_on.clone();
                    let mut new_force_off: Vec<u8> = Vec::new();
                    for pair in combination {
                        // println!("\t\tpair={:?}", pair.clone());
                        match pair.1 {
                            1 => new_force_on.push(pair.0),
                            0 => new_force_off.push(pair.0),
                            _ => panic!("this should not be possible"),
                        };
                    }
                    new_force_on.sort();
                    new_force_off.sort();

                    let mut new_val = val.clone();
                    // set all required bits
                    for &bit in new_force_on.iter().rev() {
                        new_val |= 1 << bit;
                    }
                    for &bit in new_force_off.iter().rev() {
                        let mask = 1 << bit;
                        new_val = (new_val & !mask) | ((0 << bit) & mask);
                    }

                    results.push(new_val);
                }
            }
            _ => panic!("imposta!"),
        }
        results
    }
}

fn part_1(filename: &str) -> u64 {
    let mut addresses: HashMap<u16, u64> = HashMap::new();
    let operations = read_raw_data(filename);
    let mut current_mask = operations
        .first()
        .or(Some(&Operation::Mask {
            force_on: Vec::new(),
            floating: Vec::new(),
            force_off: Vec::new(),
        }))
        .expect("expected at least 1 operation and it to be a mask")
        .clone();
    for op in operations.iter().skip(1) {
        match op {
            Operation::Mask {
                force_on,
                floating,
                force_off,
            } => {
                current_mask = Operation::Mask {
                    force_on: force_on.clone(),
                    floating: floating.clone(),
                    force_off: force_off.clone(),
                };
            }
            Operation::Set { location, value } => {
                addresses.insert(location.clone(), current_mask.apply_mask(value.clone()));
            }
        }
    }
    addresses.values().map(|&v| v as u64).sum()
}

fn part_2(filename: &str) -> u64 {
    let mut addresses: HashMap<u64, u64> = HashMap::new();
    let operations = read_raw_data(filename);
    let mut current_mask = operations
        .first()
        .or(Some(&Operation::Mask {
            force_on: Vec::new(),
            floating: Vec::new(),
            force_off: Vec::new(),
        }))
        .expect("expected at least 1 operation and it to be a mask")
        .clone();
    for op in operations.iter().skip(1) {
        match op {
            Operation::Mask {
                force_on,
                floating,
                force_off,
            } => {
                current_mask = Operation::Mask {
                    force_on: force_on.clone(),
                    floating: floating.clone(),
                    force_off: force_off.clone(),
                };
            }
            Operation::Set { location, value } => {
                for address in current_mask.apply_mask_v2(location.clone() as u64) {
                    addresses.insert(address, value.clone());
                }
            }
        }
    }
    addresses.values().map(|&v| v as u64).sum()
}

fn read_raw_data(filename: &str) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            operations.push(Operation::from(line))
        }
    }
    operations
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: u64 = 165;
        let result: u64 = super::part_1("data/example");
        assert_eq!(expected, result);

        let expected: u64 = 11926135976176;
        let result: u64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: u64 = 208;
        let result: u64 = super::part_2("data/example2");
        assert_eq!(expected, result);

        let expected: u64 = 0;
        let result: u64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
