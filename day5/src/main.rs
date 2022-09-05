use day5::line::*;
use day5::ThermalMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    let lines: Vec<Line> = reader.lines().flatten().map(|s| std::str::FromStr::from_str(s.as_ref()).unwrap()).collect();

    let map = ThermalMap::new(lines);
    println!("Number of overlapping points: {}", map.overlaps().len());

    Ok(())
}
