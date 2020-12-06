use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut running_sum: usize = 0;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("puzzles/p_6/data/input") {
        // Consumes the iterator, returns an (Optional) String
        let mut current_group: HashSet<char> = new_set();
        for line in lines {
            if let Ok(v) = line {
                if v.len() == 0 {
                    // next group
                    running_sum += current_group.len();
                    current_group = new_set();
                    continue;
                }
                let mut current_person: HashSet<char> = HashSet::new();
                for c in v.chars() {
                    current_person.insert(c);
                }

                // intersect for remaining answers
                let mut combined_group: HashSet<char> = HashSet::new();
                for answer in current_group
                    .clone()
                    .intersection(&current_person)
                    .collect::<Vec<&char>>()
                {
                    combined_group.insert(*answer);
                }
                current_group = combined_group.clone();
            }
        }
        if current_group.len() != 0 {
            running_sum += current_group.len();
            current_group.clear();
        }
    }

    println!("total sum={}", running_sum);
}

fn new_set() -> HashSet<char> {
    let mut new_group: HashSet<char> = HashSet::new();
    for c in 'a'..='z' {
        // ..= to ensure we do the full range
        new_group.insert(c);
    }
    new_group
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
