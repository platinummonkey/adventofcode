use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Default)]
struct Mountain {
    rows: Vec<Vec<String>>,
}

impl Mountain {
    fn add_row(&mut self, row: String) {
        let layout: Vec<String> = row.chars().map(|c| c.to_string()).collect();
        self.rows.push(layout);
    }

    fn get_at(&self, x: usize, y: usize) -> Option<String> {
        if y < self.rows.len() {
            let repeatable = self.rows.get(y).unwrap();
            let idx = x % repeatable.len();
            return Some(repeatable.get(idx).unwrap().clone());
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
struct Slope {
    x: usize,
    y: usize,
}

impl Slope {
    fn new(x: usize, y: usize) -> Slope {
        Slope { x, y }
    }
}

fn main() {
    // File must exist in current path before this produces output
    let mut mountain: Mountain = Mountain::default();

    if let Ok(lines) = read_lines("puzzles/p_3/data/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(v) = line {
                // do work
                mountain.add_row(v);
            }
        }
    }

    // part 1
    find_num_trees(Slope::new(3, 1), mountain.clone());

    // part 2
    let mut multiplied: i128 = 1;
    for slope in [
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ]
    .iter()
    {
        let found_trees = find_num_trees(slope.clone(), mountain.clone());
        multiplied *= found_trees;
    }
    println!("total trees in part 2 = {}", multiplied)
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

fn find_num_trees(slope: Slope, mountain: Mountain) -> i128 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut num_trees_encountered = 0;
    while let Some(coordinate) = mountain.get_at(x, y) {
        if coordinate == "#" {
            num_trees_encountered += 1;
        }
        x += slope.x;
        y += slope.y;
    }
    println!(
        "slope={:?}, num_trees_encountered={}",
        slope, num_trees_encountered
    );
    num_trees_encountered
}
