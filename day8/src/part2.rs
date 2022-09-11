use std::collections::HashMap;
use thiserror::Error;
use std::str::FromStr;
use bimap::BiHashMap;
use itertools::Itertools;

#[derive(Error, Debug)]
#[non_exhaustive]
enum Part2Error {
    #[error("Parse failure")]
    ParseError(String),
    #[error("Value already set")]
    DoubleEntry,
    #[error("Value not found")]
    ValueNotFound,
}

#[derive(Hash, Eq, PartialEq)]
enum SegmentPosition {
    Up,
    Middle,
    Down,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Wire(char);

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
struct Wires(Vec<Wire>);

impl Wires {
    fn len(&self) -> usize {
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

struct WireSegmentPositionMap(BiHashMap<Wire, SegmentPosition>);

impl WireSegmentPositionMap {
    fn new() -> Self {
        WireSegmentPositionMap(BiHashMap::new())
    }

    //fn add_new_map(&mut self, wires: Wires, val: u8) -> Result<(), Part2Error> {
    //    self.0.insert_no_overwrite(wires, val).or(Err(Part2Error::DoubleEntry))
    //}

    //fn decode(&self, wires: &Wires) -> Result<u8, Part2Error> {
    //    Ok(*self.0.get_by_left(wires).ok_or(Part2Error::ValueNotFound)?)
    //}
}

struct BrokenSevenSegmentMap(BiHashMap<Wires, u8>);

impl BrokenSevenSegmentMap {
    fn new() -> Self {
        BrokenSevenSegmentMap(BiHashMap::new())
    }

    fn add_new_map(&mut self, wires: Wires, val: u8) -> Result<(), Part2Error> {
        self.0.insert_no_overwrite(wires, val).or(Err(Part2Error::DoubleEntry))
    }

    fn decode(&self, wires: &Wires) -> Result<u8, Part2Error> {
        Ok(*self.0.get_by_left(wires).ok_or(Part2Error::ValueNotFound)?)
    }
}

struct SegmentSolver {}

impl SegmentSolver {
    fn create_map(left: &str) -> Result<BrokenSevenSegmentMap, Part2Error> {
        let mut map = BrokenSevenSegmentMap::new();

        for wires in left.trim().split(' ').map(Wires::from_str) {
            let wires = wires?;
            match wires.len() {
                2 => map.add_new_map(wires, 1)?,
                4 => map.add_new_map(wires, 4)?,
                3 => map.add_new_map(wires, 7)?,
                7 => map.add_new_map(wires, 8)?,
                _ => {}
            };
        }

        Ok(map)
    }

    fn solve_segments(line: &str) -> Result<u64, Part2Error> {
        let (map_input, seg_input) = line.split('|').collect_tuple().ok_or(Part2Error::ParseError("Couldn't parse input line".to_string()))?;
        let map = SegmentSolver::create_map(map_input);
        Ok(0)
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

    #[test]
    fn broken_seven_seg_map_add_new_map() -> Result<(), Box<dyn std::error::Error>> {
        let mut segmap = BrokenSevenSegmentMap::new();
        segmap.add_new_map(Wires::from_str("abc")?, 8)?;
        assert!(segmap.add_new_map(Wires::from_str("abc")?, 8).is_err());
        assert!(segmap.add_new_map(Wires::from_str("abc")?, 10).is_err());
        assert!(segmap.add_new_map(Wires::from_str("abcdef")?, 8).is_err());

        Ok(())
    }

    #[test]
    fn broken_seven_seg_map_decode_single() -> Result<(), Box<dyn std::error::Error>> {
        let mut segmap = BrokenSevenSegmentMap::new();
        segmap.add_new_map(Wires::from_str("acedgfb")?, 8)?;
        segmap.add_new_map(Wires::from_str("cdfbe")?, 5)?;
        segmap.add_new_map(Wires::from_str("gcdfa")?, 2)?;
        segmap.add_new_map(Wires::from_str("fbcad")?, 3)?;
        segmap.add_new_map(Wires::from_str("dab")?, 7)?;
        segmap.add_new_map(Wires::from_str("cefabd")?, 9)?;
        segmap.add_new_map(Wires::from_str("cdfgeb")?, 6)?;
        segmap.add_new_map(Wires::from_str("eafb")?, 4)?;
        segmap.add_new_map(Wires::from_str("ab")?, 1)?;

        assert_eq!(segmap.decode(&Wires::from_str("cdfeb")?)?, 5);
        assert_eq!(segmap.decode(&Wires::from_str("fcadb")?)?, 3);

        Ok(())
    }

    #[test]
    fn segment_solver_create_map() -> Result<(), Box<dyn std::error::Error>> {
        let segmap = SegmentSolver::create_map("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab")?;

        assert_eq!(segmap.decode(&Wires::from_str("acedgfb")?)?, 8);
        assert_eq!(segmap.decode(&Wires::from_str("cdfbe")?)?, 5);
        assert_eq!(segmap.decode(&Wires::from_str("gcdfa")?)?, 2);
        assert_eq!(segmap.decode(&Wires::from_str("fbcad")?)?, 3);
        assert_eq!(segmap.decode(&Wires::from_str("dab")?)?, 7);
        assert_eq!(segmap.decode(&Wires::from_str("cefabd")?)?, 9);
        assert_eq!(segmap.decode(&Wires::from_str("cdfgeb")?)?, 6);
        assert_eq!(segmap.decode(&Wires::from_str("eafb")?)?, 4);
        assert_eq!(segmap.decode(&Wires::from_str("ab")?)?, 1);

        Ok(())
    }

}
