use core::num::ParseIntError;
use std::ops::Add;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SubmarinePos {
    horizontal: i64,
    depth: i64,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubmarinePosError {
    ParseError,
}

impl SubmarinePos {
    pub fn new(horizontal: i64, depth: i64) -> Self {
        SubmarinePos { horizontal, depth }
    }
}

impl From<ParseIntError> for SubmarinePosError {
    fn from(_: ParseIntError) -> Self {
        SubmarinePosError::ParseError
    }
}

impl Add for SubmarinePos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        SubmarinePos {
            horizontal: self.horizontal + rhs.horizontal,
            depth: self.depth + rhs.depth,
        }
    }
}

impl TryFrom<&str> for SubmarinePos {
    type Error = SubmarinePosError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut sp = value.split(' ');

        match (sp.next(), sp.next()) {
            (Some("forward"), Some(horizontal)) => Ok(SubmarinePos::new(horizontal.parse::<i64>()?, 0)),
            (Some("down"), Some(depth)) => Ok(SubmarinePos::new(0, depth.parse::<i64>()?)),
            (Some("up"), Some(depth)) => Ok(SubmarinePos::new(0, -depth.parse::<i64>()?)),
            (_, _) => Err(SubmarinePosError::ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = vec!["forward 5",
                         "down 5",
                         "forward 8",
                         "up 3",
                         "down 8",
                         "forward 2"];

        let result  = input.into_iter().
            map(|line| SubmarinePos::try_from(line).unwrap() )
            .fold(SubmarinePos::new(0,0), |acc, val| acc + val );

        assert_eq!(result, SubmarinePos{horizontal: 15, depth: 10});
    }

    #[test]
    fn from_string() {
        assert_eq!(SubmarinePos::try_from("forward 5"), Ok(SubmarinePos{horizontal: 5, depth: 0}));
        assert_eq!(SubmarinePos::try_from("down 10"), Ok(SubmarinePos{horizontal: 0, depth: 10}));
        assert_eq!(SubmarinePos::try_from("up 10"), Ok(SubmarinePos{horizontal: 0, depth: -10}));

        assert_eq!(SubmarinePos::try_from("alskdjflksd 10"), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from("forward"), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from(""), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from("forward down"), Err(SubmarinePosError::ParseError));
    }
}
