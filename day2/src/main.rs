use day2::SubmarinePos;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    let mut it  = reader.lines().flatten();
    let spos = SubmarinePos::try_from(it.next().unwrap().as_ref()).unwrap();
    let result = it.fold(spos, |acc, val| acc.update(val.as_ref()).unwrap());

    println!("Result is {:?}", result);
}
