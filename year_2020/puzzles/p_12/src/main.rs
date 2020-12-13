use std::f64::consts::PI;
#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_12/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_12/data/input"));
}

fn part_1(filename: &str) -> i64 {
    let instructions = read_raw_data(filename);
    let mut current_location: Location = Location::default();

    for instruction in instructions {
        match instruction {
            Instruction::North(val) => current_location.north += val,
            Instruction::South(val) => current_location.north -= val,
            Instruction::East(val) => current_location.east += val,
            Instruction::West(val) => current_location.east -= val,
            Instruction::Left(val) => current_location.rotation += val,
            Instruction::Right(val) => current_location.rotation -= val,
            Instruction::Forward(val) => {
                let radians = (current_location.rotation as f64) * PI / 180.0;
                let east_diff = (val as f64) * radians.cos();
                let north_diff = (val as f64) * radians.sin();
                current_location.east += east_diff as i32;
                current_location.north += north_diff as i32
            }
        };
    }

    let manhattan_distance: i32 =
        i32::abs(current_location.east) + i32::abs(current_location.north);

    manhattan_distance as i64
}

fn part_2(filename: &str) -> i64 {
    let instructions = read_raw_data(filename);
    let mut current_location: Location = Location::default();
    let mut waypoint: Location = Location::default();
    waypoint.east = 10;
    waypoint.north = 1;

    for instruction in instructions {
        match instruction {
            Instruction::North(val) => waypoint.north += val,
            Instruction::South(val) => waypoint.north -= val,
            Instruction::East(val) => waypoint.east += val,
            Instruction::West(val) => waypoint.east -= val,
            Instruction::Left(degrees) => {
                // rounding issues are troublesome here.
                // these are always increments of 90 degrees
                let turns = degrees / 90;
                for _ in 0..turns {
                    let original_east = waypoint.east.clone();
                    waypoint.east = waypoint.north * -1;
                    waypoint.north = original_east;
                }
            }
            Instruction::Right(degrees) => {
                // rounding issues are troublesome here.
                // these are always increments of 90 degrees
                let turns = degrees / 90;
                for _ in 0..turns {
                    let original_east = waypoint.east.clone();
                    waypoint.east = waypoint.north;
                    waypoint.north = original_east * -1;
                }
            }
            Instruction::Forward(val) => {
                current_location.east += waypoint.east * val;
                current_location.north += waypoint.north * val
            }
        };
    }

    let manhattan_distance: i32 =
        i32::abs(current_location.east) + i32::abs(current_location.north);

    manhattan_distance as i64
}

#[derive(Debug, Copy, Clone, Default)]
struct Location {
    rotation: i32,
    east: i32,
    north: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let chars = s.chars().collect_vec();
        let val = chars
            .iter()
            .skip(1)
            .join("")
            .parse::<i32>()
            .expect("expected integer");
        match chars[0] {
            'N' => Instruction::North(val),
            'S' => Instruction::South(val),
            'W' => Instruction::West(val),
            'E' => Instruction::East(val),
            'F' => Instruction::Forward(val),
            'L' => Instruction::Left(val),
            'R' => Instruction::Right(val),
            _ => panic!("invalid data"),
        }
    }
}

fn read_raw_data(filename: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let instruction = Instruction::from(line);
            instructions.push(instruction);
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 1482;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 48739;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
