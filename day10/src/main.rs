use day10::{get_part1_score, get_part2_score};
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;

    println!("Result for part 1 is {}", get_part1_score(input.as_ref()));
    println!("Result for part 2 is {}", get_part2_score(input.as_ref()));

    Ok(())
}
