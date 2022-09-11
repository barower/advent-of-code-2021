use thiserror::Error;
use std::str::FromStr;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Part2Error {
    #[error("Parse failure")]
    ParseError(String),
    #[error("Value already set")]
    DoubleEntry,
    #[error("Value not found")]
    ValueNotFound,
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
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

#[derive(Eq, PartialEq, Hash)]
pub struct Wires(Vec<Wire>);

impl Wires {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for Wires {
    type Err = Part2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wires = s.chars().map(Wire::try_from).collect::<Result<Vec<_>, _>>()?;
        wires.sort();

        Ok(Wires(wires))
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
    fn wires_from_sorted() -> Result<(), Box<dyn std::error::Error>> {
        let wires = Wires::from_str("cafbdeg")?;
        assert_eq!(wires.0.iter().map(|w| w.0).collect::<String>(), "abcdefg");

        Ok(())
    }

}
