use day1::larger;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut data = vec![];

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        data.push(line.parse().unwrap());
    }

    println!("Result is {}", larger(data));
}
