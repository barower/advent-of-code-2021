use day6::*;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_to_string("inputfile")?;
        let fishes: Fishes = Fishes::from_str(input.trim())?;
        let result = fishes.skip(79).take(1).next().unwrap().len();

        println!("Total number of fishes: {result}");

        Ok(())
}
