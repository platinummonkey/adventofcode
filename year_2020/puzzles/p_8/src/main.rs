use util;

fn main() {
    let _ = part_1("puzzles/p_7/data/input".into());
}

fn part_1(input_file: String) -> i32 {
    if let Ok(lines) = util::iter_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            println!("{}", line);
        }
    }
    0
}
