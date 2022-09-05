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

    fn is_45_degrees(&self) -> bool {
        self.start.0.abs_diff(self.end.0) == self.start.1.abs_diff(self.end.1)
    }

    fn direction(&self) -> LineDirection {
        if self.is_horizontal() {
            if self.start.0 < self.end.0 {
                LineDirection::Right
            } else {
                LineDirection::Left
            }
        } else if self.is_vertical() {
            if self.start.1 > self.end.1 {
                LineDirection::Up
            } else {
                LineDirection::Down
            }
        } else if self.is_45_degrees() {
            match (self.start.0, self.start.1, self.end.0, self.end.1) {
                (x0, y0, x1, y1) if x0 < x1 && y0 < y1 => LineDirection::DownRight,
                (x0, y0, x1, y1) if x0 > x1 && y0 < y1 => LineDirection::DownLeft,
                (x0, y0, x1, y1) if x0 < x1 && y0 > y1 => LineDirection::UpRight,
                (x0, y0, x1, y1) if x0 > x1 && y0 > y1 => LineDirection::UpLeft,
                _ => LineDirection::Other,
            }
        } else {
            LineDirection::Other
        }
    }

    pub fn furthest_x(&self) -> usize {
        self.start.0.max(self.end.0)
    }

    pub fn furthest_y(&self) -> usize {
        self.start.1.max(self.end.1)
    }

    pub fn points(&self) -> LineIter {
        let points_left = if self.is_horizontal() {
            self.start.0.abs_diff(self.end.0) + 1
        } else if self.is_vertical() {
            self.start.1.abs_diff(self.end.1) + 1
        } else if self.is_45_degrees() {
            self.start.0.abs_diff(self.end.0) + 1
        } else { 0 };

        LineIter{
            current: self.start,
            points_left,
            direction: self.direction(),
        }
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

#[derive(Debug, PartialEq, Eq)]
enum LineDirection {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineIter {
    current: (usize, usize),
    points_left: usize,
    direction: LineDirection,
}

impl Iterator for LineIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.points_left > 0 {
            let to_return = self.current;

            self.current = match self.direction {
                LineDirection::Up => (self.current.0, self.current.1-1),
                LineDirection::Down => (self.current.0, self.current.1+1),
                LineDirection::Left => (self.current.0-1, self.current.1),
                LineDirection::Right => (self.current.0+1, self.current.1),

                LineDirection::UpRight => (self.current.0+1, self.current.1-1),
                LineDirection::UpLeft => (self.current.0-1, self.current.1-1),
                LineDirection::DownRight => (self.current.0+1, self.current.1+1),
                LineDirection::DownLeft => (self.current.0-1, self.current.1+1),

                _ => { return None; },
            };
            self.points_left -= 1;

            Some(to_return)
        } else {
            None
        }
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
    fn line_45_degrees() {
        assert!(!Line{start: (15,15), end: (18,15)}.is_45_degrees());
        assert!(Line{start: (15,15), end: (15,15)}.is_45_degrees());
        assert!(Line{start: (15,15), end: (20,20)}.is_45_degrees());
        assert!(Line{start: (20,20), end: (15,15)}.is_45_degrees());
        assert!(Line{start: (5,0), end: (0,5)}.is_45_degrees());
    }

    #[test]
    fn line_direction() {
        assert_eq!(Line{start: (15,15), end: (18,15)}.direction(), LineDirection::Right);
        assert_eq!(Line{start: (18,15), end: (15,15)}.direction(), LineDirection::Left);
        assert_eq!(Line{start: (15,15), end: (15,10)}.direction(), LineDirection::Up);
        assert_eq!(Line{start: (15,10), end: (15,15)}.direction(), LineDirection::Down);

        assert_eq!(Line{start: (10,10), end: (15,15)}.direction(), LineDirection::DownRight);
        assert_eq!(Line{start: (15,15), end: (10,10)}.direction(), LineDirection::UpLeft);
        assert_eq!(Line{start: (0,5), end: (5,0)}.direction(), LineDirection::UpRight);
        assert_eq!(Line{start: (5,0), end: (0,5)}.direction(), LineDirection::DownLeft);

        assert_eq!(Line{start: (10,10), end: (15,16)}.direction(), LineDirection::Other);
    }

    #[test]
    fn line_create_iter() {
        assert_eq!(
            Line{start: (15,15), end: (18,15)}.points(),
            LineIter{ current: (15,15), points_left: 4, direction: LineDirection::Right}
        );

        assert_eq!(
            Line{start: (18,15), end: (15,15)}.points(),
            LineIter{ current: (18,15), points_left: 4, direction: LineDirection::Left}
        );

        assert_eq!(
            Line{start: (15,15), end: (15,10)}.points(),
            LineIter{ current: (15,15), points_left: 6, direction: LineDirection::Up}
        );

        assert_eq!(
            Line{start: (15,10), end: (15,15)}.points(),
            LineIter{ current: (15,10), points_left: 6, direction: LineDirection::Down}
        );

        assert_eq!(
            Line{start: (10,10), end: (15,15)}.points(),
            LineIter{ current: (10,10), points_left: 6, direction: LineDirection::DownRight },
        );

        assert_eq!(
            Line{start: (15,15), end: (10,10)}.points(),
            LineIter{ current: (15,15), points_left: 6, direction: LineDirection::UpLeft },
        );

        assert_eq!(
            Line{start: (0,5), end: (5,0)}.points(),
            LineIter{ current: (0,5), points_left: 6, direction: LineDirection::UpRight },
        );

        assert_eq!(
            Line{start: (5,0), end: (0,5)}.points(),
            LineIter{ current: (5,0), points_left: 6, direction: LineDirection::DownLeft },
        );

    }

    #[test]
    fn line_iter() {
        let mut up_iter = LineIter{current:(5,5), points_left: 3, direction: LineDirection::Up};
        assert_eq!(up_iter.next(), Some((5,5)));
        assert_eq!(up_iter.next(), Some((5,4)));
        assert_eq!(up_iter.next(), Some((5,3)));
        assert_eq!(up_iter.next(), None);

        let mut diagonal_iter = LineIter{current:(1,1), points_left: 4, direction: LineDirection::DownRight};
        assert_eq!(diagonal_iter.next(), Some((1,1)));
        assert_eq!(diagonal_iter.next(), Some((2,2)));
        assert_eq!(diagonal_iter.next(), Some((3,3)));
        assert_eq!(diagonal_iter.next(), Some((4,4)));
        assert_eq!(diagonal_iter.next(), None);
    }
}
