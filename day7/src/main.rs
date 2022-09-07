use day7::*;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_to_string("inputfile")?;
        let crabs: Crabs = Crabs::from_str(input.trim())?;
        let result = crabs.cheapest_cost();

        println!("Minimum fuel cost: {result}");

        Ok(())
}
