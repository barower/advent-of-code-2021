use day9::Heatmap;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;
    let heatmap = Heatmap::from_str(&input)?;

    let x: u64 = heatmap.find_low_points().iter().map(|(x,y)| (1 + heatmap.get_at((*x).try_into().unwrap(), (*y).try_into().unwrap()).unwrap()) as u64).sum();
    println!("Result of part 1 is {}", x);

    let mut p2_results: Vec<_> = heatmap.find_all_basins().into_iter().map(|set| set.len()).collect();
    p2_results.sort();
    p2_results = p2_results.into_iter().rev().collect();

    println!("Result of part 2 is {}", p2_results[0] * p2_results[1] * p2_results[2]);

    Ok(())
}
