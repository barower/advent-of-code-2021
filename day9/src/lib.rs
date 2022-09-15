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

pub struct Heatmap(Vec<Vec<u8>>);

impl Heatmap {
    pub fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut retvec = vec![];

        for (y, row) in self.0.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let x: isize = x.try_into().unwrap();
                let y: isize = y.try_into().unwrap();

                if self.is_local_minimum(x, y) {
                    retvec.push((x as usize, y as usize));
                }
            }
        }

        retvec
    }

    pub fn find_all_basins(&self) -> Vec<HashSet<(isize, isize)>> {
        let mut retvec = vec![];

        for (x, y) in self.find_low_points().iter() {
            retvec.push(self.find_basin(((*x).try_into().unwrap(), (*y).try_into().unwrap())));
        }

        retvec
    }

    fn find_basin(&self, (x, y): (isize, isize)) -> HashSet<(isize, isize)> {
        let mut basin: HashSet<(isize, isize)> = HashSet::new();
        let mut candidates: HashSet<(isize, isize)> = vec![(x, y)].into_iter().collect();

        while !candidates.is_empty() {
            let mut new_candidates: HashSet<(isize, isize)> = HashSet::new();

            for (x, y) in candidates.iter() {
                if let Some(val) = self.get_at(*x, *y) {
                    if !basin.contains(&(*x,*y)) && val < 9 {
                        basin.insert((*x, *y));
                        for (relx, rely) in [(0, 1), (-1, 0), (1, 0), (0, -1)] {
                            new_candidates.insert((*x+relx, *y+rely));
                        }
                    }
                }
            }

            candidates = new_candidates;
        }

        basin
    }

    fn is_local_minimum(&self, x: isize, y: isize) -> bool {
        let candidate = match self.get_at(x,y) {
            Some(candidate) => candidate,
            None => { return false; },
        };

        for (relx, rely) in [(0, 1), (-1, 0), (1, 0), (0, -1)] {
            if let Some(neighbour) = self.get_at(x+relx, y+rely) {
                if candidate >= neighbour {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_at(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            None
        } else {
            Some(self.0[y as usize][x as usize])
        }
    }

    fn width(&self) -> isize {
        self.0[0].len().try_into().unwrap()
    }

    fn height(&self) -> isize {
        self.0.len().try_into().unwrap()
    }
}

impl FromStr for Heatmap {
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

        Ok(Heatmap(retvec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_heatmap() -> Heatmap {
        Heatmap::from_str("2199943210
3987894921
9856789892
8767896789
9899965678").unwrap()
    }

    #[test]
    fn heatmap_from_str() {
        let heatmap = get_test_heatmap();

        assert_eq!(heatmap.width(), 10);
        assert_eq!(heatmap.height(), 5);

        assert_eq!(heatmap.get_at(0, 0), Some(2));
        assert_eq!(heatmap.get_at(1, 0), Some(1));
        assert_eq!(heatmap.get_at(0, 1), Some(3));
    }

    #[test]
    fn heatmap_is_local_minimum() {
        let heatmap = get_test_heatmap();

        assert!(!heatmap.is_local_minimum(0, 0));
        assert!(heatmap.is_local_minimum(1, 0));
        assert!(heatmap.is_local_minimum(9, 0));
        assert!(!heatmap.is_local_minimum(5, 5));
        assert!(heatmap.is_local_minimum(2, 2));
        assert!(!heatmap.is_local_minimum(0, 4));
        assert!(heatmap.is_local_minimum(6, 4));

    }

    #[test]
    fn heatmap_find_low_points() {
        let heatmap = get_test_heatmap();
        assert_eq!(heatmap.find_low_points(), vec![(1,0), (9,0), (2,2), (6,4)]);
    }

    #[test]
    fn heatmap_find_basin() {
        let heatmap = get_test_heatmap();
        assert_eq!(heatmap.find_basin((0, 0)), vec![(0,0), (1,0), (0,1)].into_iter().collect());

        assert_eq!(heatmap.find_basin((9, 0)).len(), 9);
        assert_eq!(heatmap.find_basin((0, 3)).len(), 14);
        assert_eq!(heatmap.find_basin((9, 4)).len(), 9);
    }
}
