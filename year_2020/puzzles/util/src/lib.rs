use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::path::Path;

pub use itertools::Itertools;
pub use regex::Regex;

pub use std::collections::{HashMap, HashSet};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct InputLineReader {
    buf: io::Lines<io::BufReader<File>>,
}

impl Iterator for InputLineReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next() {
            Some(line) => match line {
                Ok(l) => Some(l),
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[allow(dead_code)]
pub fn iter_lines<P: AsRef<Path>>(filename: P) -> Result<InputLineReader, io::Error> {
    match read_lines(filename) {
        Err(e) => Err(e),
        Ok(lines) => Ok(InputLineReader { buf: lines }),
    }
}
