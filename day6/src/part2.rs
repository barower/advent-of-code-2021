use std::str::FromStr;

// Each index represents days left
pub struct Fishes([u64; 9]);

impl FromStr for Fishes {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut retarr = [0; 9];
        let fishes = s.split(',').map(|val| val.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
        for fish in fishes {
            retarr[fish] += 1;
        }

        Ok(Fishes(retarr))
    }
}

impl Iterator for Fishes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
        Some(self.0.iter().sum())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part2_from_str() -> Result<(), Box<dyn std::error::Error>> {
        let input = "0,0,5,5,5,8";
        let fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.0, [2, 0, 0, 0, 0, 3, 0, 0, 1]);

        Ok(())
    }
    #[test]

    fn part2_next() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let mut fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.next(), Some(5));
        assert_eq!(fishes.next(), Some(6));
        assert_eq!(fishes.next(), Some(7));

        Ok(())
    }

    #[test]
    fn part2_after_18_days() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.skip(17).take(1).next(), Some(26));

        Ok(())
    }

    #[test]
    fn part2_after_80_days() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.skip(79).take(1).next(), Some(5934));

        Ok(())
    }
}
