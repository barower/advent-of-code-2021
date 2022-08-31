use core::num::ParseIntError;
use std::ops::Add;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SubmarinePos {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubmarinePosError {
    ParseError,
}

impl SubmarinePos {
    pub fn new(horizontal: i64, depth: i64, aim: i64) -> Self {
        SubmarinePos { horizontal, depth, aim }
    }

    pub fn update(self, value: &str) -> Result<SubmarinePos, SubmarinePosError> {
        let mut sp = value.split(' ');

        match (sp.next(), sp.next()) {
            (Some("forward"), Some(x)) => {
                Ok(SubmarinePos::new(self.horizontal + x.parse::<i64>()?, self.depth + x.parse::<i64>()? * self.aim, self.aim))
            },
            (Some("down"), Some(aim)) => {
                Ok(SubmarinePos::new(self.horizontal, self.depth, self.aim + aim.parse::<i64>()?))
            },
            (Some("up"), Some(aim)) => {
                Ok(SubmarinePos::new(self.horizontal, self.depth, self.aim - aim.parse::<i64>()?))
            },
            (_, _) => Err(SubmarinePosError::ParseError),
        }
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
            aim: self.aim + rhs.aim,
        }
    }
}

impl TryFrom<&str> for SubmarinePos {
    type Error = SubmarinePosError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut sp = value.split(' ');

        match (sp.next(), sp.next()) {
            (Some("forward"), Some(horizontal)) => Ok(SubmarinePos::new(horizontal.parse::<i64>()?, 0, 0)),
            (Some("down"), Some(aim)) => Ok(SubmarinePos::new(0, 0, aim.parse::<i64>()?)),
            (Some("up"), Some(aim)) => Ok(SubmarinePos::new(0, 0, -aim.parse::<i64>()?)),
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

        let mut it  = input.into_iter();
        let spos = SubmarinePos::try_from(it.next().unwrap()).unwrap();
        let result = it.fold(spos, |acc, val| acc.update(val).unwrap());

        assert_eq!(result, SubmarinePos{horizontal: 15, depth: 60, aim: 10});
    }

    #[test]
    fn from_string() {
        assert_eq!(SubmarinePos::try_from("forward 5"), Ok(SubmarinePos{horizontal: 5, depth: 0, aim: 0}));
        assert_eq!(SubmarinePos::try_from("down 10"), Ok(SubmarinePos{horizontal: 0, depth: 0, aim: 10}));
        assert_eq!(SubmarinePos::try_from("up 10"), Ok(SubmarinePos{horizontal: 0, depth: 0, aim: -10}));

        assert_eq!(SubmarinePos::try_from("alskdjflksd 10"), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from("forward"), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from(""), Err(SubmarinePosError::ParseError));
        assert_eq!(SubmarinePos::try_from("forward down"), Err(SubmarinePosError::ParseError));
    }
}
