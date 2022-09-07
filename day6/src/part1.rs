use std::str::FromStr;

pub struct Fishes(Vec<u64>);

impl FromStr for Fishes {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes = s.split(',').map(|val| val.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;

        Ok(Fishes(fishes))
    }
}

impl Iterator for Fishes {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut born_fish_counter = 0;

        for fish_days_left in self.0.iter_mut() {
            *fish_days_left = match *fish_days_left {
                0 => { born_fish_counter += 1; 6 },
                _ => { *fish_days_left -1 },
            }
        }

        for _ in 0..born_fish_counter {
            self.0.push(8);
        }

        Some(self.0.clone())
    }
}

pub fn bench_function() -> Result<(), Box<dyn std::error::Error>> {
    let input = "3,4,3,1,2";
    let fishes: Fishes = Fishes::from_str(input)?;
    assert_eq!(fishes.skip(100).take(1).next().unwrap().len(), 36920);

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part1_next() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let mut fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.next(), Some(vec![2,3,2,0,1]));
        assert_eq!(fishes.next(), Some(vec![1,2,1,6,0,8]));
        assert_eq!(fishes.next(), Some(vec![0,1,0,5,6,7,8]));

        Ok(())
    }

    #[test]
    fn part1_after_18_days() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.skip(17).take(1).next(), Some(vec![6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8]));

        Ok(())
    }

    #[test]
    fn part1_after_80_days() -> Result<(), Box<dyn std::error::Error>> {
        let input = "3,4,3,1,2";
        let fishes: Fishes = Fishes::from_str(input)?;
        assert_eq!(fishes.skip(79).take(1).next().unwrap().len(), 5934);

        Ok(())
    }
}
