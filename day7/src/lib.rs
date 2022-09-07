use std::str::FromStr;

pub struct Crabs(Vec<u64>);

impl FromStr for Crabs {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crabs = s.split(',').map(|val| val.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;

        Ok(Crabs(crabs))
    }
}

impl Crabs {
    fn part1_cost_to(&self, position: u64) -> u64 {
        self.0.iter().map(|pos| pos.abs_diff(position)).sum()
    }

    fn part2_cost_to(&self, position: u64) -> u64 {
        self.0.iter().map(|pos| (1..pos.abs_diff(position)+1).sum::<u64>()).sum()
    }

    pub fn part1_cheapest_cost(&self) -> u64 {
        (0..self.0.iter().max().unwrap()+1).map(|pos| self.part1_cost_to(pos)).min().unwrap()
    }

    pub fn part2_cheapest_cost(&self) -> u64 {
        (0..self.0.iter().max().unwrap()+1).map(|pos| self.part2_cost_to(pos)).min().unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn from_str() -> Result<(), Box<dyn std::error::Error>> {
        let input = "0,0,5,5,5,8";
        let crabs: Crabs = Crabs::from_str(input)?;
        assert_eq!(crabs.0, vec![0,0,5,5,5,8]);

        Ok(())
    }

    #[test]
    fn part1_cost_to() -> Result<(), Box<dyn std::error::Error>> {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs: Crabs = Crabs::from_str(input)?;
        assert_eq!(crabs.part1_cost_to(2), 37);
        assert_eq!(crabs.part1_cost_to(1), 41);
        assert_eq!(crabs.part1_cost_to(3), 39);
        assert_eq!(crabs.part1_cost_to(10), 71);

        Ok(())
    }

    #[test]
    fn part1_cheapest_cost() -> Result<(), Box<dyn std::error::Error>> {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs: Crabs = Crabs::from_str(input)?;
        assert_eq!(crabs.part1_cheapest_cost(), 37);

        Ok(())
    }

    #[test]
    fn part2_cost_to() -> Result<(), Box<dyn std::error::Error>> {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs: Crabs = Crabs::from_str(input)?;
        assert_eq!(crabs.part2_cost_to(5), 168);

        Ok(())
    }
    #[test]
    fn part2_cheapest_cost() -> Result<(), Box<dyn std::error::Error>> {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs: Crabs = Crabs::from_str(input)?;
        assert_eq!(crabs.part2_cheapest_cost(), 168);

        Ok(())
    }

}

