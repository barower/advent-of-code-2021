use day9::Heatmap;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;
    let heatmap = Heatmap::from_str(&input)?;

    let x: u64 = heatmap.find_low_points().iter().map(|val| (1 + *val) as u64).sum();
    println!("Result of part 1 is {}", x);

    Ok(())
}
