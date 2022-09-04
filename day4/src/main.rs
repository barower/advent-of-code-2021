use day4::Game;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut data: Vec<String> = vec![];

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        data.push(line.parse().unwrap());
    }
}
