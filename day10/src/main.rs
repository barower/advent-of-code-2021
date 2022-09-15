use day10::get_total_score;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;

    println!("Result for part 1 is {}", get_total_score(input.as_ref()));

    Ok(())
}
