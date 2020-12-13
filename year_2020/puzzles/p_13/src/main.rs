#[allow(unused_imports)]
use util::*;

fn main() {
    println!("part 1 = {}", part_1("puzzles/p_13/data/input"));
    println!("part 2 = {}", part_2("puzzles/p_13/data/input"));
}

fn part_1(filename: &str) -> i64 {
    let (depart_time, bus_times) = read_raw_data(filename);
    println!("depart_time={} bus_times={:?}", depart_time, bus_times);
    let mut closest_bus_time_id: i64 = 0;
    for bus_time in bus_times {
        println!(
            "bus_time={} closest_bus_time_id={}",
            bus_time, closest_bus_time_id
        );
        if depart_time % bus_time == 0 {
            closest_bus_time_id = bus_time;
            break;
        }
        if closest_bus_time_id == 0 {
            closest_bus_time_id = bus_time
        } else if depart_time % bus_time > depart_time % closest_bus_time_id {
            closest_bus_time_id = bus_time
        }
    }
    let minutes_waiting = closest_bus_time_id - depart_time % closest_bus_time_id;
    closest_bus_time_id * minutes_waiting
}

fn part_2(filename: &str) -> i64 {
    let (_, bus_times) = read_raw_data(filename);

    0
}

fn read_raw_data(filename: &str) -> (i64, Vec<i64>) {
    let mut depart_time: i64 = 0;
    let mut times: Vec<i64> = Vec::new();
    if let Ok(lines) = util::iter_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if i == 0 {
                depart_time = line.parse::<i64>().expect("invalid first line");
            } else {
                times = line
                    .split(",")
                    .filter(|&x| x != "x")
                    .map(|x| x.parse::<i64>().expect("invalid integer"))
                    .collect_vec()
            }
        }
    }
    (depart_time, times)
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
