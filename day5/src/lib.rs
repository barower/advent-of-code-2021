mod line;

use std::fmt::{Display, Formatter};

use line::*;

struct ThermalMap(Vec<Vec<u64>>);

impl ThermalMap {
    fn new(lines: Vec<Line>) -> Self {
        let furthest_x: usize = lines.iter().fold(0, |max, line| if line.furthest_x() > max { line.furthest_x() } else { max });
        let furthest_y: usize = lines.iter().fold(0, |max, line| if line.furthest_y() > max { line.furthest_y() } else { max });

        let mut column_vec: Vec<Vec<u64>> = Vec::new();
        column_vec.reserve_exact(furthest_y+1);
        for _ in 0..(furthest_y+1) {
            column_vec.push(vec![0; furthest_x+1]);
        }

        ThermalMap(column_vec)
    }

    fn overlaps(&self) -> Vec<(usize, usize)> {
        vec![]
    }
}

impl Display for ThermalMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

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

        assert_eq!(map.0[4][4], 1);
        assert_eq!(map.0[5][4], 1);
        assert_eq!(map.0[6][4], 1);

        assert_eq!(map.0[3][3], 0);
        assert_eq!(map.0[0][0], 0);
        assert_eq!(map.0[6][0], 0);
    }

    //#[test]
    //fn thermal_map_multiple_lines() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

    //#[test]
    //fn thermal_map_diagonal_line() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

    //#[test]
    //fn thermal_map_multiple_lines_with_diagonals() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

    //#[test]
    //fn thermal_map_display() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

    //#[test]
    //fn thermal_map_overlaps() {
    //    let line = Line{start: (99,100), end: (100,100)};
    //    let map = ThermalMap::new(vec![line]);
    //}

}
