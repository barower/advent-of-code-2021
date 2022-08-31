use day2::SubmarinePos;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day2(data: Vec<String>) -> SubmarinePos {
    //data.into_iter().
    //    map(|line| SubmarinePos::try_from(line).unwrap() )
    //    .fold(SubmarinePos::new(0,0), |acc, val| acc + val )
    todo!()
}

fn main() {
    let mut data: Vec<String> = vec![];

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        data.push(line.clone());
    }

    println!("Result is {:?}", day2(data));
}
