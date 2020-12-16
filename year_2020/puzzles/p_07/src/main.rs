use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // noop
    println!(
        "part 1 answer: {}",
        part_1("puzzles/p_07/data/input".into())
    );
    println!(
        "part 2 answer: {}",
        part_2("puzzles/p_07/data/input".into())
    );
}

#[derive(Debug, Clone, Default)]
struct Bag {
    id: String,
    contains: HashMap<String, i32>,
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for fun visual display
        let mut buf: String = String::new();
        buf.push_str(format!("id={}\n", self.id).as_str());
        for child in self.contains.iter() {
            buf.push_str(format!("\t{} {}\n", child.1, child.0).as_str());
        }
        write!(f, "{}", buf)
    }
}

impl Bag {
    fn new(line: String) -> Bag {
        let mut b = Bag::default();

        let mut parts = line.split(|c| c == ' ' || c == ',' || c == '.');
        let parts = parts.by_ref();
        b.id = parts
            .take(2)
            .map(|w| w.into())
            .collect::<Vec<String>>()
            .join(" ");
        // skip the next two
        if parts.next() != Some("bags") {
            panic!("expected 'bags', invalid input: {}", line)
        }
        if parts.next() != Some("contain") {
            panic!("expected 'bags', invalid input: {}", line)
        }
        // "bag contains"
        b.contains = HashMap::new();
        // <num> <id> bag(s)[,...].
        loop {
            let child_bag_qty = match parts.next() {
                None => break,
                Some("no") => break,
                Some("") | Some("bag") | Some("bags") => continue,
                Some(child_bag_qty) => child_bag_qty,
            }
            .parse::<i32>()
            .unwrap();
            let child_bag_id: String = parts
                .take(2)
                .map(|w| w.into())
                .collect::<Vec<String>>()
                .join(" ");
            b.contains.insert(child_bag_id, child_bag_qty);
        }
        // println!("created {}\n", b);
        b
    }

    fn can_directly_hold_id(&self, bag_id: String) -> bool {
        if self.id.to_string() == bag_id {
            return false;
        }
        for child in self.contains.iter() {
            if child.0.to_string() == bag_id {
                return true;
            }
        }
        false
    }
}

fn part_1(input_file: String) -> i32 {
    // File must exist in current path before this produces output
    let mut bags: HashMap<String, Bag> = HashMap::new();
    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                // do stuff
                let b = Bag::new(v);
                bags.insert(b.clone().id, b);
            }
        }
    }

    // solve this backwards, we assume there are no cycles! :see_no_evil:
    let mut available_options: HashSet<String> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut required: HashSet<String> = HashSet::new();
    for bag in bags.iter() {
        if bag.1.can_directly_hold_id("shiny gold".to_string()) {
            visited.insert(bag.0.to_string());
            required.insert(bag.0.to_string());
            available_options.insert(bag.0.to_string());
        }
    }
    loop {
        if required.len() == 0 {
            break;
        }
        let mut new_required: HashSet<String> = HashSet::new();
        for parent in required.clone().drain() {
            for bag in bags.iter() {
                if !visited.contains(bag.0) && bag.1.can_directly_hold_id(parent.clone()) {
                    new_required.insert(bag.0.to_string());
                    available_options.insert(bag.0.to_string());
                }
            }
            visited.insert(parent.to_string());
        }
        required = new_required;
    }

    available_options.len() as i32
}

fn part_2(input_file: String) -> i32 {
    // File must exist in current path before this produces output
    let mut bags: HashMap<String, Bag> = HashMap::new();
    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                // do stuff
                let b = Bag::new(v);
                bags.insert(b.clone().id, b);
            }
        }
    }

    let mut total_count: i32 = 0;

    let mut required: VecDeque<(String, i32)> = VecDeque::new();
    required.push_front(("shiny gold".to_string(), 1));

    while let Some((bag_id, count)) = required.pop_back() {
        let bag = bags.get(bag_id.as_str());
        match bag {
            None => continue,
            Some(bag) => {
                for (child_bag_id, child_count) in bag.contains.clone() {
                    required.push_front((child_bag_id, child_count * count));
                    total_count += child_count * count;
                }
            }
        }
    }

    total_count
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

#[test]
fn test_part_1() {
    let expected: i32 = 148;
    let result: i32 = part_1("data/input".into());
    assert_eq!(expected, result);
}

#[test]
fn test_part_2() {
    let expected: i32 = 24867;
    let result: i32 = part_2("data/input".into());
    assert_eq!(expected, result);
}
