pub mod line;

//use std::fmt::{Display, Formatter};

use line::*;

pub struct ThermalMap(Vec<Vec<u64>>);

impl ThermalMap {
    pub fn new(lines: Vec<Line>) -> Self {
        let furthest_x: usize = lines.iter().fold(0, |max, line| if line.furthest_x() > max { line.furthest_x() } else { max });
        let furthest_y: usize = lines.iter().fold(0, |max, line| if line.furthest_y() > max { line.furthest_y() } else { max });

        let mut column_vec: Vec<Vec<u64>> = Vec::new();
        column_vec.reserve_exact(furthest_y+1);
        for _ in 0..(furthest_y+1) {
            column_vec.push(vec![0; furthest_x+1]);
        }

        for line in &lines {
            for (x, y) in line.points() {
                column_vec[y][x] += 1;
            }
        }

        ThermalMap(column_vec)
    }

    fn at(&self, x: usize, y: usize) -> u64 {
        self.0[y][x]
    }

    pub fn overlaps(&self) -> Vec<(usize, usize)> {
        let mut retvec = vec![];

        for (y, col) in self.0.iter().enumerate() {
            for (x, _) in col.iter().enumerate() {
                if self.at(x, y) > 1 {
                    retvec.push((x, y));
                }
            }
        }

        retvec
    }
}

//impl Display for ThermalMap {
//    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//        todo!()
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermal_map_dimensions() {
        let line = line::Line::new((99,100), (100,100));
        let map = ThermalMap::new(vec![line]);
        assert_eq!(map.0.len(), 101);
        for row_vec in map.0.iter() {
            assert_eq!(row_vec.len(), 101);
        }
    }

    #[test]
    fn thermal_map_single_nondiagonal_line() {
        let line = line::Line::new((4,4), (6,4));
        let map = ThermalMap::new(vec![line]);

        assert_eq!(map.at(4,4), 1);
        assert_eq!(map.at(5,4), 1);
        assert_eq!(map.at(6,4), 1);

        assert_eq!(map.at(3,4), 0);
        assert_eq!(map.at(0,0), 0);
        assert_eq!(map.at(6,0), 0);
    }

    #[test]
    fn thermal_map_multiple_lines() {
        let line0 = line::Line::new((4,4), (6,4));
        let line1 = line::Line::new((5,3), (5,6));
        let map = ThermalMap::new(vec![line0, line1]);

        assert_eq!(map.at(4,4), 1);
        assert_eq!(map.at(5,4), 2);
        assert_eq!(map.at(6,4), 1);

        assert_eq!(map.at(5,3), 1);
        assert_eq!(map.at(5,6), 1);

        assert_eq!(map.at(3,4), 0);
        assert_eq!(map.at(0,0), 0);
        assert_eq!(map.at(6,0), 0);
    }

    #[test]
    fn thermal_map_diagonal_line() {
        let line = line::Line::new((1,1), (5,5));
        let map = ThermalMap::new(vec![line]);

        for i in 0..=5 {
            for j in 0..=5 {
                assert_eq!(map.at(i,j), 0);
            }
        }
    }

    #[test]
    fn thermal_map_multiple_lines_with_diagonals() {
        let line0 = line::Line::new((4,4), (6,4));
        let line1 = line::Line::new((5,3), (5,6));
        let line2 = line::Line::new((1,1), (5,5));
        let map = ThermalMap::new(vec![line0, line1, line2]);

        assert_eq!(map.at(4,4), 1);
        assert_eq!(map.at(5,4), 2);
        assert_eq!(map.at(6,4), 1);

        assert_eq!(map.at(5,3), 1);
        assert_eq!(map.at(5,6), 1);

        assert_eq!(map.at(3,4), 0);
        assert_eq!(map.at(0,0), 0);
        assert_eq!(map.at(6,0), 0);
    }

    //#[test]
    //fn thermal_map_display() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

    #[test]
    fn thermal_map_overlaps() {
        let line0 = line::Line::new((4,4), (6,4));
        let line1 = line::Line::new((5,3), (5,6));
        let line2 = line::Line::new((1,1), (5,5));
        let line3 = line::Line::new((5,6), (5,8));
        let map = ThermalMap::new(vec![line0, line1, line2, line3]);

        assert_eq!(map.overlaps(), vec![(5,4), (5,6)])
    }

}

