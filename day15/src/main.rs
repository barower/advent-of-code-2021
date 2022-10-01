use day15::RiskMap;
use std::str::FromStr;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("inputfile")?;
    let mut riskmap: RiskMap = RiskMap::from_str(input.trim())?;
    let mut destination = (riskmap.width()-1, riskmap.height()-1);

    println!("Part 1's result is {:?}", riskmap.dijkstra_lowest_risk_path((0, 0), destination));

    riskmap.expand_right(4);
    riskmap.expand_down(4);

    destination = (riskmap.width()-1, riskmap.height()-1);

    println!("Part 2's result is {:?}", riskmap.dijkstra_lowest_risk_path((0, 0), destination));

    Ok(())
}
