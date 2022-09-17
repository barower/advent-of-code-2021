use std::str::FromStr;
use day11::EnergyMap;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;
    let mut energymap = EnergyMap::from_str(&input)?;
    let mut total_flash_count = 0;

    for i in 0.. {
        let current_flash_count = energymap.step();
        if current_flash_count == (energymap.width() * energymap.height()).try_into()? {
            println!("All octopuses flashed at i = {}", i+1);
            return Ok(());
        }
        total_flash_count += current_flash_count;
        if i == 99 {
            println!("Result of part 1 is {total_flash_count}");
        }
    }

    Ok(())
}
