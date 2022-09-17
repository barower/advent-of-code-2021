use std::str::FromStr;
use thiserror::Error;
use std::collections::HashSet;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Day8Error {
    #[error("Parse error")]
    ParseError,
    #[error("Unknown error")]
    UnknownError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnergyMap(Vec<Vec<u8>>);

impl EnergyMap {
    fn flash(&mut self, (x, y): (isize, isize)) {
        for (relx, rely) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            if let Some(energy) = self.get_ref_mut(x+relx, y+rely) {
                *energy += 1;
            }
        }
    }

    pub fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        for (_, line) in self.0.iter_mut().enumerate() {
            for (_, energy) in line.iter_mut().enumerate() {
                *energy += 1;
            }
        }

        // Then, any octopus with an energy level greater than 9 flashes.
        let mut all_flashes: HashSet<(usize, usize)> = HashSet::new();
        let mut remaining_flashes: HashSet<(usize, usize)> = HashSet::new();

        for (y, line) in self.0.iter().enumerate() {
            for (x, energy) in line.iter().enumerate() {
                if *energy > 9 {
                    all_flashes.insert((x, y));
                    remaining_flashes.insert((x, y));
                }
            }
        }

        while !remaining_flashes.is_empty() {
            let mut new_remaining_flashes = HashSet::new();

            for (x, y) in remaining_flashes.iter() {
                self.flash(((*x).try_into().unwrap(), (*y).try_into().unwrap()));
            }

            for (y, line) in self.0.iter().enumerate() {
                for (x, energy) in line.iter().enumerate() {
                    if *energy > 9 && !all_flashes.contains(&(x, y)){
                        all_flashes.insert((x, y));
                        new_remaining_flashes.insert((x, y));
                    }
                }
            }

            remaining_flashes = new_remaining_flashes;
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0
        for (x, y) in all_flashes.iter() {
            self.0[*y][*x] = 0;
        }

        all_flashes.len()
    }

    pub fn get_ref_mut(&mut self, x: isize, y: isize) -> Option<&mut u8> {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            None
        } else {
            Some(&mut self.0[y as usize][x as usize])
        }
    }

    fn width(&self) -> isize {
        self.0[0].len().try_into().unwrap()
    }

    fn height(&self) -> isize {
        self.0.len().try_into().unwrap()
    }
}

impl FromStr for EnergyMap {
    type Err = Day8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(Day8Error::UnknownError)?.len();
        let height = s.lines().count();

        let mut retvec = vec![vec![0; width]; height];

        for (y, line) in s.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                retvec[y][x] = chr.to_digit(10).ok_or(Day8Error::ParseError)?.try_into().or(Err(Day8Error::ParseError))?;
            }
        }

        Ok(EnergyMap(retvec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energymap_from_str() {
        let mut energymap = EnergyMap::from_str("2199943210
3987894921
9856789892
8767896789
9899965678").unwrap();

        assert_eq!(energymap.width(), 10);
        assert_eq!(energymap.height(), 5);

        assert_eq!(energymap.get_ref_mut(0, 0), Some(&mut 2));
        assert_eq!(energymap.get_ref_mut(1, 0), Some(&mut 1));
        assert_eq!(energymap.get_ref_mut(0, 1), Some(&mut 3));
    }

    #[test]
    fn energymap_step() {
        let mut energymap = EnergyMap::from_str("11111
19991
19191
19991
11111").unwrap();

        assert_eq!(energymap.step(), 9);
        assert_eq!(energymap, EnergyMap::from_str("34543
40004
50005
40004
34543").unwrap());

        assert_eq!(energymap.step(), 0);
        assert_eq!(energymap, EnergyMap::from_str("45654
51115
61116
51115
45654").unwrap());

    }
}
