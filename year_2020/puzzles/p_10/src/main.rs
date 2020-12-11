#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_10/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_10/data/input"));
}

fn part_1(filename: &str) -> i64 {
    let mut one_jolt_diffs: i64 = 1;
    let mut three_volt_diffs: i64 = 1;
    let mut data = read_raw_data(filename);
    data.sort();
    data.windows(2).map(|w| w[1] - w[0]).for_each(|c| {
        match c {
            1 => one_jolt_diffs += 1,
            3 => three_volt_diffs += 1,
            _ => {}
        };
    });
    one_jolt_diffs * three_volt_diffs
}

fn part_2(filename: &str) -> i64 {
    let mut data = read_raw_data(filename);
    data.sort();

    // end conditions
    data.insert(0, data[0]);
    data.push(data[data.len() - 1] + 3);

    // with_capacity here makes push much more efficient for known sizes.
    let mut sums: Vec<i64> = Vec::with_capacity(data.len());
    for _ in 0..data.len() {
        sums.push(0);
    }
    sums[0] = 1;

    for i in (0 as i32)..(data.len() as i32) {
        for j in 1..=3 {
            if i - j >= 0 && data[i as usize] - data[(i - j) as usize] <= 3 {
                sums[i as usize] += sums[(i - j) as usize]
            }
        }
    }
    sums[data.len() - 1]
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
        let expected: i64 = 1625;
        let result: i64 = super::part_1("data/input");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_2() {
        let expected: i64 = 3100448333024;
        let result: i64 = super::part_2("data/input");
        assert_eq!(expected, result);
    }
}
