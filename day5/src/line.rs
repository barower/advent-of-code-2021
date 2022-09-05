use thiserror::Error;
use std::str::FromStr;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LineError {
    #[error("Could not parse line")]
    ParseError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Line{start, end}
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn furthest_x(&self) -> usize {
        self.start.0.max(self.end.0)
    }

    pub fn furthest_y(&self) -> usize {
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


}
