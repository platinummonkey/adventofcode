use std::collections::VecDeque;
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_09/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_09/data/input"));
}

fn part_1(filename: &str) -> i64 {
    let (mut preamble, data) = read_data(filename);
    for x in data {
        if !preamble
            .iter()
            .cartesian_product(preamble.iter()) // all possible sums
            .filter(|(a, b)| a != b) // don't allow same numbers rule
            /*.map(|(a, b)| {
                println!("a={} b={} x={}", a, b, x);
                (a, b)
            })*/
            .any(|(&a, &b)| a + b == x)
        {
            return x;
        }
        preamble.pop_front();
        preamble.push_back(x);
    }
    -1
}

fn part_2(filename: &str) -> i64 {
    let magical_num = part_1(filename.clone());
    let data = read_raw_data(filename);
    /* LOL this part was just for fun, it's terribly inefficient
    let alt_answer = (0..data.len())
        .enumerate()
        .cartesian_product((0..data.len()).enumerate())
        .filter(|(a, b)| a.0 < b.0)
        .filter(|(a, b)| {
            data.iter()
                .skip(a.clone().0)
                .take(b.clone().0 - a.clone().0 + 1)
                .sum::<i64>()
                == magical_num
        })
        .map(|(a, b)| {
            let minmax = data
                .iter()
                .skip(a.clone().0)
                .take(b.clone().0 - a.clone().0 + 1)
                .minmax()
                .into_option()
                .expect("expected option");
            minmax.0 + minmax.1
        })
        .next()
        .expect("expected alt answer");
    println!("alt answer={}", alt_answer);
    */

    for i in 0..data.len() {
        let mut sum = data[i];
        for j in i + 1..data.len() {
            sum += data[j];
            if sum == magical_num {
                let (min_val, max_val) = data
                    .iter()
                    .skip(i)
                    .take(j - i + 1)
                    .minmax()
                    .into_option()
                    .expect("expected a range");
                return min_val + max_val;
            }
        }
    }

    0
}

fn read_data(filename: &str) -> (VecDeque<i64>, Vec<i64>) {
    let mut preamble: VecDeque<i64> = VecDeque::new();
    let mut data: Vec<i64> = Vec::new();
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        lines
            .map(|line| {
                line.parse::<i64>()
                    .expect(format!("invalid integer!: {}", line).as_str())
            })
            .enumerate()
            .for_each(|(i, val)| match i {
                0..=24 => preamble.push_back(val),
                _ => data.push(val),
            });
    }
    (preamble, data)
}

fn read_raw_data(filename: &str) -> Vec<i64> {
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        return lines
            .map(|line| {
                line.parse::<i64>()
                    .expect(format!("invalid integer!: {}", line).as_str())
            })
            .collect::<Vec<i64>>();
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let expected: i64 = 69316178;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 9351526;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
