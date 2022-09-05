use std::str::FromStr;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LineError {
    #[error("Could not parse line")]
    ParseError,
}

#[derive(Debug, PartialEq)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn furthest_x(&self) -> usize {
        self.start.0.max(self.end.0)
    }

    fn furthest_y(&self) -> usize {
        self.start.1.max(self.end.1)
    }
}

impl FromStr for Line {
    type Err = LineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(" -> ");

        let mut first_point_sp = sp.next().ok_or(LineError::ParseError)?.split(',');
        let x0 = first_point_sp.next().ok_or(LineError::ParseError)?.parse::<usize>().or(Err(LineError::ParseError))?;
        let y0 = first_point_sp.next().ok_or(LineError::ParseError)?.parse::<usize>().or(Err(LineError::ParseError))?;

        let mut second_point_sp = sp.next().ok_or(LineError::ParseError)?.split(',');
        let x1 = second_point_sp.next().ok_or(LineError::ParseError)?.parse::<usize>().or(Err(LineError::ParseError))?;
        let y1 = second_point_sp.next().ok_or(LineError::ParseError)?.parse::<usize>().or(Err(LineError::ParseError))?;

        Ok(Line{start: (x0,y0), end: (x1,y1)})
    }
}

enum LineIterDirection {
    Up,
    Down,
    Left,
    Right,
}

struct LineIter {
    start: (usize, usize),
    points_lefs: usize,
    direction: LineIterDirection,
}

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
    fn line_from_str() -> Result<(), Box<dyn std::error::Error>> {

        let input = "25,4 -> 21,37";

        let parsed_line: Line = FromStr::from_str(input)?;
        assert_eq!(parsed_line, Line{start: (25,4), end: (21,37)});

        Ok(())
    }

    #[test]
    fn line_horizontal() {
        assert!(Line{start: (15,15), end: (18,15)}.is_horizontal());
        assert!(Line{start: (15,15), end: (15,15)}.is_horizontal());
        assert!(!Line{start: (15,15), end: (15,18)}.is_horizontal());
        assert!(!Line{start: (15,15), end: (18,18)}.is_horizontal());
    }

    #[test]
    fn line_vertical() {
        assert!(!Line{start: (15,15), end: (18,15)}.is_vertical());
        assert!(Line{start: (15,15), end: (15,15)}.is_vertical());
        assert!(Line{start: (15,15), end: (15,18)}.is_vertical());
        assert!(!Line{start: (15,15), end: (18,18)}.is_vertical());
    }

    #[test]
    fn thermal_map_dimensions() {
        let line = Line{start: (99,100), end: (100,100)};
        let map = ThermalMap::new(vec![line]);
        assert_eq!(map.0.len(), 101);
        for row_vec in map.0.iter() {
            assert_eq!(row_vec.len(), 101);
        }
    }

    #[test]
    fn thermal_map_single_nondiagonal_line() {
        let line = Line{start: (4,4), end: (6,4)};
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

