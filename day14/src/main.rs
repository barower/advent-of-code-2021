use std::fs::File;
use std::io::{BufRead, BufReader};

use day14::rules::Rules;
use day14::Polymer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules = Rules::new();
    let mut polymer = Polymer::new("OFSVVSFOCBNONHKFHNPK");

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        rules.add(line.as_ref())?;
    }

    for _ in 0..10 {
        polymer = polymer.step(&rules);
    }

    let sorted_occurencies = polymer.sorted_char_occurences();
    let (_, most) = sorted_occurencies.last().unwrap();
    let (_, least) = sorted_occurencies.first().unwrap();

    println!("Result of part 1 is {}", most - least);

    Ok(())
}
