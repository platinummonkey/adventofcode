use std::borrow::Borrow;
use std::convert::Into;
use util::*;

fn main() {
    println!(
        "part 1 accumulator value={}",
        part_1("puzzles/p_8/data/input".into())
    );
    println!(
        "part 2 accumulator value={}",
        part_2("puzzles/p_8/data/input".into())
    );
}

#[derive(Debug, Clone, Default)]
struct Instruction {
    cmd: String,
    number: i32,
}

impl Instruction {
    fn new(cmd: String, number: i32) -> Instruction {
        Instruction { cmd, number }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum RunResult {
    Invalid(i32),
    LoopDetected(i32),
    Terminated(i32),
}

impl Into<i32> for RunResult {
    fn into(self) -> i32 {
        match self {
            RunResult::Invalid(num) => num,
            RunResult::LoopDetected(num) => num,
            RunResult::Terminated(num) => num,
        }
    }
}

fn read_instructions(input_file: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    if let Ok(lines) = util::iter_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if line.len() == 0 {
                instructions.push(Instruction::new("nop".into(), 0));
                continue;
            }
            let command: (&str, i32) = line
                .split_ascii_whitespace()
                .tuples()
                .map(|(instruction, number)| {
                    (
                        instruction,
                        number.parse::<i32>().expect("integer was malformed"),
                    )
                })
                .next()
                .expect("expected a valid line");
            // dbg!(command);
            instructions.push(Instruction::new(command.0.into(), command.1));
        }
    }
    instructions
}

fn part_1(input_file: String) -> i32 {
    let instructions = read_instructions(input_file);
    let accumulator_value = run_instructions(&instructions);
    accumulator_value.into()
}

fn part_2(input_file: String) -> i32 {
    let mut instructions = read_instructions(input_file);
    // brute force... there is probably a smarter branching algorithm to find these easier, but OoF.
    for i in 0..instructions.len() {
        let mutated_instruction = match instructions[i].cmd.clone().as_str() {
            "nop" => Instruction::new("jmp".to_string(), instructions[i].number),
            "jmp" => Instruction::new("nop".to_string(), instructions[i].number),
            _ => continue,
        };
        let original = instructions.remove(i);
        instructions.insert(i, mutated_instruction);

        match run_instructions(instructions.clone().as_slice()) {
            RunResult::Terminated(val) => {
                dbg!(i);
                return val;
            }
            _ => {
                instructions.remove(i);
                instructions.insert(i, original);
            }
        }
    }
    -1
}

fn run_instructions(instructions: &[Instruction]) -> RunResult {
    let mut accumulator_value: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut line_no: i32 = 0;
    loop {
        if line_no < 0 {
            return RunResult::Invalid(accumulator_value);
        }
        if line_no >= instructions.len() as i32 {
            // we reached the end
            return RunResult::Terminated(accumulator_value);
        }
        if visited.contains(line_no.clone().borrow()) {
            // loop detected
            return RunResult::LoopDetected(accumulator_value);
        }
        visited.insert(line_no);
        let instruction = instructions
            .get(line_no as usize)
            .expect("invalid instruction does not exist!");
        match instruction.cmd.as_str() {
            "acc" => {
                // println!("acc {}", instruction.number);
                accumulator_value += instruction.number;
                line_no += 1;
            }
            "jmp" => {
                // println!("jmp {}", instruction.number);
                line_no += instruction.number;
            }
            _ => {
                // println!("nop {}", instruction.number);
                line_no += 1;
                // assume nop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i32 = 1654;
        let result: i32 = super::part_1("data/input".into());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i32 = 833;
        let result: i32 = super::part_2("data/input".into());
        assert_eq!(expected, result);
    }
}
