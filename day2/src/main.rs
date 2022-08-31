use day2::SubmarinePos;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    let result = reader.lines().flatten()
            .map(|line| SubmarinePos::try_from(line.as_ref()).unwrap())
            .fold(SubmarinePos::new(0,0), |acc, val| acc + val);

    println!("Result is {:?}", result);
}
