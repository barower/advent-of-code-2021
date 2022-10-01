use std::fs::File;
use std::io::{BufRead, BufReader};

use day14::part1::NaivePolymer;
use day14::part2::OptimalPolymer;
use day14::polymer::*;
use day14::rules::Rules;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules = Rules::new();
    let mut part1polymer = NaivePolymer::new("OFSVVSFOCBNONHKFHNPK");
    let mut part2polymer = OptimalPolymer::new("OFSVVSFOCBNONHKFHNPK");

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        rules.add(line.as_ref())?;
    }

    for _ in 0..10 {
        part1polymer = part1polymer.step(&rules);
    }

    let sorted_occurencies = sorted_char_occurences(part1polymer);
    let (_, most) = sorted_occurencies.last().unwrap();
    let (_, least) = sorted_occurencies.first().unwrap();

    println!("Result of part 1 is {}", most - least);

    for _ in 0..40 {
        part2polymer = part2polymer.step(&rules);
    }

    let sorted_occurencies = sorted_char_occurences(part2polymer);
    let (_, most) = sorted_occurencies.last().unwrap();
    let (_, least) = sorted_occurencies.first().unwrap();

    println!("Result of part 1 is {}", most - least);
    Ok(())
}
