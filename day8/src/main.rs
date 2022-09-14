use day8::part1;
use day8::part2::SegmentSolver;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().flatten().collect();

    let result = part1(&lines);
    println!("Result for part 1 is {result}");

    let result2: u64 = lines.iter().map(std::ops::Deref::deref).map(SegmentSolver::solve_segments).collect::<Result<Vec<_>, _>>()?.iter().sum();
    println!("Result for part 2 is {result2}");

    Ok(())
}
