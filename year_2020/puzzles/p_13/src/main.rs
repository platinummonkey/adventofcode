#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_13/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_13/data/input"));
}

fn part_1(_filename: &str) -> i64 {
    0
}

fn part_2(_filename: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 0;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 0;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
