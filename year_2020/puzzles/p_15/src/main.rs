#[allow(unused_imports)]
use util::*;

fn main() {
    let input: Vec<i64> = vec![8, 11, 0, 19, 1, 2];
    println!("part 1 = {}", solution(input.clone(), 2020));
    println!("part 2 = {}", solution(input.clone(), 30000000));
}

fn solution(input: Vec<i64>, iter_count: i64) -> i64 {
    let mut seen: HashMap<i64, i64> = HashMap::new();
    let mut next_val: i64 = 0;
    for (i, v) in input.iter().enumerate() {
        let turn = (i + 1) as i64;
        match seen.get(v).cloned() {
            Some(previous_value) => {
                seen.insert(*v, turn);
                next_val = turn - previous_value;
            }
            None => {
                seen.insert(*v, turn);
                next_val = 0;
            }
        }
    }

    for turn in (input.len() as i64 + 1)..iter_count {
        match seen.get(&next_val).cloned() {
            Some(previous_val) => {
                seen.insert(next_val, turn);
                next_val = turn - previous_val;
            }
            None => {
                seen.insert(next_val, turn);
                next_val = 0;
            }
        }
    }
    next_val
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 436;
        let result: i64 = super::solution(vec![0, 3, 6], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 1;
        let result: i64 = super::solution(vec![1, 3, 2], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 10;
        let result: i64 = super::solution(vec![2, 1, 3], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 27;
        let result: i64 = super::solution(vec![1, 2, 3], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 78;
        let result: i64 = super::solution(vec![2, 3, 1], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 438;
        let result: i64 = super::solution(vec![3, 2, 1], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 1836;
        let result: i64 = super::solution(vec![3, 1, 2], 2020);
        assert_eq!(expected, result);

        let expected: i64 = 447;
        let result: i64 = super::solution(vec![8, 11, 0, 19, 1, 2], 2020);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 175594;
        let result: i64 = super::solution(vec![0, 3, 6], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 2578;
        let result: i64 = super::solution(vec![1, 3, 2], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 3544142;
        let result: i64 = super::solution(vec![2, 1, 3], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 261214;
        let result: i64 = super::solution(vec![1, 2, 3], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 6895259;
        let result: i64 = super::solution(vec![2, 3, 1], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 18;
        let result: i64 = super::solution(vec![3, 2, 1], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 362;
        let result: i64 = super::solution(vec![3, 1, 2], 30000000);
        assert_eq!(expected, result);

        let expected: i64 = 11721679;
        let result: i64 = super::solution(vec![8, 11, 0, 19, 1, 2], 30000000);
        assert_eq!(expected, result);
    }
}
