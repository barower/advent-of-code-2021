mod neighbours;
mod dijkstra;

use neighbours::NeighboursIterator;
use dijkstra::*;
use std::str::FromStr;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Day15Error {
    #[error("Parse error")]
    ParseError,
    #[error("Unknown error")]
    UnknownError,
}

pub struct RiskMap(Vec<Vec<u8>>);

impl RiskMap {
    pub fn dijkstra_lowest_risk_path(&self, from: (usize, usize), to: (usize, usize)) -> Weight<u64> {
        let mut dijkstra_map = DijkstraMap::new(self.width(), self.height(), from);

        let mut next_point = Some(from);

        while let Some(point) = next_point {
            let current_weight = {
                if let Weight::NonInf(weight) = dijkstra_map.get_weight(point) {
                    weight
                } else {
                    panic!("Impossible node weight");
                }
            };

            for neighbour in NeighboursIterator::new(point, self.width(), self.height()) {
                dijkstra_map.update(neighbour, point, current_weight + self.get_at(neighbour) as u64)
            }

            dijkstra_map.mark_visited(point);

            next_point = dijkstra_map.get_smallest_non_visited();
        }

        dijkstra_map.get_weight(to)
    }

    pub fn get_path_cost(&self, path: &[(usize, usize)]) -> u64 {
        path.iter().fold(0, |acc, (x, y)| acc + self.0[*y][*x] as u64)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    fn get_at(&self, (x, y): (usize, usize)) -> u8 {
        self.0[y][x]
    }
}

impl FromStr for RiskMap {
    type Err = Day15Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or(Day15Error::UnknownError)?.len();
        let height = s.lines().count();

        let mut retvec: Vec<Vec<u8>> = vec![vec![0; width]; height];

        for (y, line) in s.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                retvec[y][x] = chr
                    .to_digit(10)
                    .ok_or(Day15Error::ParseError)?
                    .try_into()
                    .or(Err(Day15Error::ParseError))?;
            }
        }

        Ok(RiskMap(retvec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_riskmap() -> RiskMap {
        RiskMap::from_str(
            "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
        )
        .unwrap()
    }

    #[test]
    fn riskmap_dimensions() {
        let riskmap = get_test_riskmap();

        assert_eq!(riskmap.width(), 10);
        assert_eq!(riskmap.height(), 10);
    }

    #[test]
    fn riskmap_get_path_cost() {
        let riskmap = get_test_riskmap();

        assert_eq!(riskmap.get_path_cost(&[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]), 8);
    }

    #[test]
    fn riskmap_dijkstra() {
        let riskmap = get_test_riskmap();

        assert_eq!(riskmap.dijkstra_lowest_risk_path((0, 0), (9, 9)), Weight::NonInf(40));
    }
}
