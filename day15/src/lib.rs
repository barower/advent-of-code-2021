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

#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn expand_right(&mut self, mut times: usize) {
        let mut new_copy = self.clone();
        while times > 0 {
            new_copy.increment();
            self.append_right(&new_copy);
            times -= 1;
        }
    }

    pub fn expand_down(&mut self, mut times: usize) {
        let mut new_copy = self.clone();
        while times > 0 {
            new_copy.increment();
            self.append_down(&new_copy);
            times -= 1;
        }
    }

    fn get_at(&self, (x, y): (usize, usize)) -> u8 {
        self.0[y][x]
    }

    fn increment(&mut self) {
        for val in self.0.iter_mut().flatten() {
            if *val == 9 {
                *val = 1;
            } else {
                *val += 1;
            }
        }
    }

    fn append_right(&mut self, other: &Self) {
        let old_width = self.width();
        let new_width = old_width + other.width();

        for row in self.0.iter_mut() {
            row.resize(new_width, 0);
        }

        for (appy, appended_row) in other.0.iter().enumerate() {
            for (appx, appended_val) in appended_row.iter().enumerate() {
                self.0[appy][old_width + appx] = *appended_val;
            }
        }
    }

    fn append_down(&mut self, other: &Self) {
        let old_height = self.height();
        let new_height = old_height + other.height();

        self.0.resize(new_height, vec![0; self.width()]);

        for (appy, appended_row) in other.0.iter().enumerate() {
            for (appx, appended_val) in appended_row.iter().enumerate() {
                self.0[old_height + appy][appx] = *appended_val;
            }
        }
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

    #[test]
    fn riskmap_increment() {
        let mut riskmap = get_test_riskmap();
        riskmap.increment();

        let expected = RiskMap::from_str("2274862853
2492484783
3247622439
4715142671
8574528222
2421239248
2461123532
4236532741
2314249632
3422155692").unwrap();

        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_append_right() {
        let mut riskmap = get_test_riskmap();

        let to_append = RiskMap::from_str("2274862853
2492484783
3247622439
4715142671
8574528222
2421239248
2461123532
4236532741
2314249632
3422155692").unwrap();

        riskmap.append_right(&to_append);

        let expected = RiskMap::from_str("11637517422274862853
13813736722492484783
21365113283247622439
36949315694715142671
74634171118574528222
13191281372421239248
13599124212461123532
31254216394236532741
12931385212314249632
23119445813422155692").unwrap();


        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_expand_right_once() {
        let mut riskmap = get_test_riskmap();

        riskmap.expand_right(1);

        let expected = RiskMap::from_str("11637517422274862853
13813736722492484783
21365113283247622439
36949315694715142671
74634171118574528222
13191281372421239248
13599124212461123532
31254216394236532741
12931385212314249632
23119445813422155692").unwrap();


        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_expand_right_twice() {
        let mut riskmap = get_test_riskmap();

        riskmap.expand_right(2);

        let expected = RiskMap::from_str("116375174222748628533385973964
138137367224924847833513595894
213651132832476224394358733541
369493156947151426715826253782
746341711185745282229685639333
131912813724212392483532341359
135991242124611235323572234643
312542163942365327415347643852
129313852123142496323425351743
231194458134221556924533266713").unwrap();

        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_append_down() {
        let mut riskmap = get_test_riskmap();

        let to_append = RiskMap::from_str("2274862853
2492484783
3247622439
4715142671
8574528222
2421239248
2461123532
4236532741
2314249632
3422155692").unwrap();

        riskmap.append_down(&to_append);

        let expected = RiskMap::from_str("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
2274862853
2492484783
3247622439
4715142671
8574528222
2421239248
2461123532
4236532741
2314249632
3422155692").unwrap();

        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_expand_down_once() {
        let mut riskmap = get_test_riskmap();

        riskmap.expand_down(1);

        let expected = RiskMap::from_str("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
2274862853
2492484783
3247622439
4715142671
8574528222
2421239248
2461123532
4236532741
2314249632
3422155692").unwrap();

        assert_eq!(riskmap, expected);
    }

    #[test]
    fn riskmap_dijkstra_expanded() {
        let mut riskmap = get_test_riskmap();

        riskmap.expand_right(4);
        riskmap.expand_down(4);

        assert_eq!(riskmap.dijkstra_lowest_risk_path((0, 0), (49, 49)), Weight::NonInf(315));
    }

}
