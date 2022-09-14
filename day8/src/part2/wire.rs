use thiserror::Error;
use std::fmt;
use std::str::FromStr;
use std::ops::{Add, Sub};

#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Part2Error {
    #[error("Parse failure")]
    ParseError(String),
    #[error("Value already set")]
    DoubleEntry,
    #[error("Value not found")]
    ValueNotFound,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct Wire(char);

impl TryFrom<char> for Wire {
    type Error = Part2Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='g' => Ok(Wire(c)),
            _ => Err(Part2Error::ParseError("Invalid character".to_string()))
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Wires(Vec<Wire>);

impl Wires {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn empty() -> Wires {
        Wires(vec![])
    }
}

impl FromStr for Wires {
    type Err = Part2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wires = s.chars().map(Wire::try_from).collect::<Result<Vec<_>, _>>()?;
        wires.sort();
        wires.dedup();

        Ok(Wires(wires))
    }
}

impl fmt::Display for Wires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_write: String = self.0.iter().map(|ch| ch.0).collect();
        write!(f, "{}", to_write)
    }
}

impl Add<Wires> for Wires {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut wires = self.0.into_iter().chain(rhs.0.into_iter()).collect::<Vec<_>>();
        wires.sort();
        wires.dedup();

        Wires(wires)
    }
}

impl Sub<Wires> for Wires {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = self;
        ret.0.retain(|elem| !rhs.0.contains(elem));
        ret
    }
}

impl Add<Wire> for Wires {
    type Output = Self;

    fn add(self, rhs: Wire) -> Self::Output {
        let mut ret = self;
        ret.0.push(rhs);
        ret.0.sort();
        ret.0.dedup();

        ret
    }
}

impl Sub<Wire> for Wires {
    type Output = Self;

    fn sub(self, rhs: Wire) -> Self::Output {
        let mut ret = self;
        ret.0.retain(|elem| elem != &rhs);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_from() -> Result<(), Box<dyn std::error::Error>> {
        let w = Wire::try_from('a')?; assert_eq!(w.0, 'a');
        let w = Wire::try_from('b')?; assert_eq!(w.0, 'b');
        let w = Wire::try_from('c')?; assert_eq!(w.0, 'c');
        let w = Wire::try_from('d')?; assert_eq!(w.0, 'd');
        let w = Wire::try_from('e')?; assert_eq!(w.0, 'e');
        let w = Wire::try_from('f')?; assert_eq!(w.0, 'f');
        let w = Wire::try_from('g')?; assert_eq!(w.0, 'g');

        assert!(Wire::try_from('h').is_err());
        assert!(Wire::try_from('0').is_err());
        assert!(Wire::try_from('A').is_err());
        assert!(Wire::try_from('F').is_err());

        Ok(())
    }

    #[test]
    fn wires_from() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("abcdefg")?;
        assert_eq!(wires.0.iter().map(|w| w.0).collect::<String>(), "abcdefg");

        Ok(())
    }

    #[test]
    fn wires_from_empty() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("")?;
        assert_eq!(wires.len(), 0);

        Ok(())
    }

    #[test]
    fn wires_from_sorted() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("cafbdeg")?;
        assert_eq!(wires.0.iter().map(|w| w.0).collect::<String>(), "abcdefg");

        Ok(())
    }

    #[test]
    fn wires_from_duplicated() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("bccad")?;
        assert_eq!(wires.0.iter().map(|w| w.0).collect::<String>(), "abcd");

        Ok(())
    }

    #[test]
    fn wires_add_wires() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("ba")?;
        let wires2 = Wires::from_str("ac")?;
        assert_eq!(wires + wires2, Wires::from_str("abc")?);

        Ok(())
    }

    #[test]
    fn wires_sub_wires() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("abdf")?;
        let wires2 = Wires::from_str("ac")?;
        assert_eq!(wires - wires2, Wires::from_str("bdf")?);

        Ok(())
    }

    #[test]
    fn wires_add_wire() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("b")?;
        let wire = Wire::try_from('a')?;
        assert_eq!(wires + wire, Wires::from_str("ab")?);

        let wires = Wires::from_str("a")?;
        let wire = Wire::try_from('a')?;
        assert_eq!(wires + wire, Wires::from_str("a")?);

        Ok(())
    }

    #[test]
    fn wires_sub_wire() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("abc")?;
        let wire = Wire::try_from('b')?;
        assert_eq!(wires - wire, Wires::from_str("ac")?);

        let wires = Wires::from_str("a")?;
        let wire = Wire::try_from('a')?;
        assert_eq!(wires - wire, Wires::from_str("")?);

        Ok(())
    }
}
