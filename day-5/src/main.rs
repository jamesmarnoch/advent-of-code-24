use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(fs::File::open("input.txt").unwrap());

    let lines = buf_reader.lines();
}
