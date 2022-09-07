use day6::*;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_to_string("inputfile")?;
        let fishes: part1::Fishes = part1::Fishes::from_str(input.trim())?;
        let result = fishes.skip(79).take(1).next().unwrap().len();

        println!("Total number of fishes: {result}");

        let fishes: part2::Fishes = part2::Fishes::from_str(input.trim())?;
        let result = fishes.skip(255).take(1).next().unwrap();
        println!("Total number of fishes after 256 days: {result}");

        Ok(())
}
