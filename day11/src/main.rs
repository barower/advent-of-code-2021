use std::str::FromStr;
use day11::EnergyMap;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;
    let mut energymap = EnergyMap::from_str(&input)?;
    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += energymap.step();
    }

    println!("Result of part 1 is {flash_count}");

    Ok(())
}
