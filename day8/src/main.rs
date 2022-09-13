//use day8::part1;
use day8::part2::wire::*;
use std::str::FromStr;
//use std::fs::File;
//use std::io::{BufRead, BufReader};

fn find_zero() -> Result<Wires, Part2Error> {
    let one: Wires = Wires::from_str("cf")?;
    let four: Wires = Wires::from_str("bcdf")?;
    let seven: Wires = Wires::from_str("acf")?;
    let eight: Wires = Wires::from_str("abcdefg")?;

    let x = eight - (four - one);

    let rest = vec![Wires::from_str("abcefg")?,
    Wires::from_str("acdeg")?,
    Wires::from_str("acdfg")?,
    Wires::from_str("abdfg")?,
    Wires::from_str("abdefg")?,
    Wires::from_str("abcdfg")?];

    for inp in rest.iter() {
        if (x.clone() - inp.clone()) == (Wires::from_str("")?) {
            return Ok(inp.clone());
        }
    }

    Err(Part2Error::ValueNotFound)
}

fn find_what() -> Result<Wires, Part2Error> {
    let one: Wires = Wires::from_str("cf")?;
    let four: Wires = Wires::from_str("bcdf")?;
    let seven: Wires = Wires::from_str("acf")?;
    let eight: Wires = Wires::from_str("abcdefg")?;
    let zero: Wires = Wires::from_str("abcefg")?;

    let x = eight - (four - one);

    let rest = vec![Wires::from_str("acdeg")?,
    Wires::from_str("acdfg")?,
    Wires::from_str("abdfg")?,
    Wires::from_str("abdefg")?,
    Wires::from_str("abcdfg")?];

    for inp in rest.iter() {
        if (x.clone() - inp.clone()) == (Wires::from_str("")?) {
            return Ok(inp.clone());
        }
    }

    Err(Part2Error::ValueNotFound)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let file = File::open("inputfile").expect("Failed to find file with input data");
    //let reader = BufReader::new(file);
    //let lines: Vec<_> = reader.lines().flatten().collect();

    //let result = part1(&lines);
    //println!("Result for part 1 is {result}");

    println!("Found zero: {:?}", find_zero());

    Ok(())
}
