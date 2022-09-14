#![allow(unused)]

pub mod wire;

use wire::*;
use bimap::BiHashMap;
use itertools::Itertools;
use std::str::FromStr;

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

struct WireSegmentPositionMap(BiHashMap<wire::Wire, SegmentPosition>);

impl WireSegmentPositionMap {
    fn new() -> Self {
        WireSegmentPositionMap(BiHashMap::new())
    }

    fn add_new_map(&mut self, wire: wire::Wire, pos: SegmentPosition) -> Result<(), Part2Error> {
        self.0.insert_no_overwrite(wire, pos).or(Err(Part2Error::DoubleEntry))
    }

    fn wire_from_position(&self, pos: &SegmentPosition) -> Result<wire::Wire, Part2Error> {
        Ok(*self.0.get_by_right(pos).ok_or(Part2Error::ValueNotFound)?)
    }
}

struct BrokenSevenSegmentMap(BiHashMap<wire::Wires, u8>);

impl BrokenSevenSegmentMap {
    fn new() -> Self {
        BrokenSevenSegmentMap(BiHashMap::new())
    }

    fn add_new_map(&mut self, wires: wire::Wires, val: u8) -> Result<(), Part2Error> {
        self.0.insert_no_overwrite(wires, val).or(Err(Part2Error::DoubleEntry))
    }

    fn decode(&self, wires: &wire::Wires) -> Result<u8, Part2Error> {
        Ok(*self.0.get_by_left(wires).ok_or(Part2Error::ValueNotFound)?)
    }

    fn from_value(&self, value: u8) -> Option<wire::Wires> {
        self.0.get_by_right(&value).cloned()
    }
}

pub struct SegmentSolver {}

impl SegmentSolver {
    fn create_map(left: &str) -> Result<BrokenSevenSegmentMap, Part2Error> {
        let mut map = BrokenSevenSegmentMap::new();

        // Find 1, 4, 7 and 8
        for wires in left.trim().split(' ').map(wire::Wires::from_str) {
            let wires = wires?;
            match wires.len() {
                2 => map.add_new_map(wires, 1)?,
                4 => map.add_new_map(wires, 4)?,
                3 => map.add_new_map(wires, 7)?,
                7 => map.add_new_map(wires, 8)?,
                _ => {}
            };
        }

        // Find 6 and 9
        for wires in left.trim().split(' ').map(wire::Wires::from_str) {
            let wires = wires?;
            if map.decode(&wires).is_err() {
                let four: Wires = map.from_value(4).ok_or(Part2Error::ValueNotFound)?;
                let seven: Wires = map.from_value(7).ok_or(Part2Error::ValueNotFound)?;
                let eight: Wires = map.from_value(8).ok_or(Part2Error::ValueNotFound)?;

                // Find 6
                if (wires.clone() + seven) == eight {
                    map.add_new_map(wires, 6)?;
                    continue;
                }

                // Find 9
                if (wires.clone() + four) == wires {
                    map.add_new_map(wires, 9)?;
                    continue;
                }
            }
        }

        // Find 5
        for wires in left.trim().split(' ').map(wire::Wires::from_str) {
            let wires = wires?;
            if map.decode(&wires).is_err() {
                let six: Wires = map.from_value(6).ok_or(Part2Error::ValueNotFound)?;

                // Find 5
                if (wires.clone() - six) == Wires::empty() {
                    map.add_new_map(wires, 5)?;
                    continue;
                }
            }
        }

        // Find 3
        for wires in left.trim().split(' ').map(wire::Wires::from_str) {
            let wires = wires?;
            if map.decode(&wires).is_err() {
                let nine: Wires = map.from_value(9).ok_or(Part2Error::ValueNotFound)?;

                // Find 3
                if (wires.clone() - nine) == Wires::empty() {
                    map.add_new_map(wires, 3)?;
                    continue;
                }
            }
        }

        // Find 2 and 0
        for wires in left.trim().split(' ').map(wire::Wires::from_str) {
            let wires = wires?;
            if map.decode(&wires).is_err() {
                let three: Wires = map.from_value(3).ok_or(Part2Error::ValueNotFound)?;

                // Find 2
                if (wires.clone() - three).len() == 1 {
                    map.add_new_map(wires, 2)?;
                    continue;
                } else { // Find 0
                    map.add_new_map(wires, 0)?;
                    continue;
                }
            }
        }

        Ok(map)
    }

    pub fn solve_segments(line: &str) -> Result<u64, Part2Error> {
        let (map_input, seg_input) = line.split('|').collect_tuple().ok_or(Part2Error::ParseError("Couldn't parse input line".to_string()))?;
        let map = SegmentSolver::create_map(map_input)?;
        let mut result = 0;
        for (power, wire) in seg_input.trim().split(' ').rev().enumerate() {
            let x: u64 = map.decode(&wire::Wires::from_str(wire)?)?.into();
            result += 10_u64.pow(power.try_into().unwrap()) * x;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_segment_add_new_map() -> Result<(), Box<dyn std::error::Error>>{

        let mut wiremap = WireSegmentPositionMap::new();
        wiremap.add_new_map(wire::Wire::try_from('a')?, SegmentPosition::Up)?;

        assert!(wiremap.add_new_map(wire::Wire::try_from('a')?, SegmentPosition::Up).is_err());

        assert!(wiremap.add_new_map(wire::Wire::try_from('b')?, SegmentPosition::Up).is_err());
        assert!(wiremap.add_new_map(wire::Wire::try_from('a')?, SegmentPosition::LowerLeft).is_err());

        wiremap.add_new_map(wire::Wire::try_from('b')?, SegmentPosition::Down)?;

        Ok(())
    }

    #[test]
    fn broken_seven_seg_map_add_new_map() -> Result<(), Box<dyn std::error::Error>> {
        let mut segmap = BrokenSevenSegmentMap::new();
        segmap.add_new_map(wire::Wires::from_str("abc")?, 8)?;
        assert!(segmap.add_new_map(wire::Wires::from_str("abc")?, 8).is_err());
        assert!(segmap.add_new_map(wire::Wires::from_str("abc")?, 10).is_err());
        assert!(segmap.add_new_map(wire::Wires::from_str("abcdef")?, 8).is_err());

        Ok(())
    }

    #[test]
    fn broken_seven_seg_map_decode_single() -> Result<(), Box<dyn std::error::Error>> {
        let mut segmap = BrokenSevenSegmentMap::new();
        segmap.add_new_map(wire::Wires::from_str("acedgfb")?, 8)?;
        segmap.add_new_map(wire::Wires::from_str("cdfbe")?, 5)?;
        segmap.add_new_map(wire::Wires::from_str("gcdfa")?, 2)?;
        segmap.add_new_map(wire::Wires::from_str("fbcad")?, 3)?;
        segmap.add_new_map(wire::Wires::from_str("dab")?, 7)?;
        segmap.add_new_map(wire::Wires::from_str("cefabd")?, 9)?;
        segmap.add_new_map(wire::Wires::from_str("cdfgeb")?, 6)?;
        segmap.add_new_map(wire::Wires::from_str("eafb")?, 4)?;
        segmap.add_new_map(wire::Wires::from_str("ab")?, 1)?;

        assert_eq!(segmap.decode(&wire::Wires::from_str("cdfeb")?)?, 5);
        assert_eq!(segmap.decode(&wire::Wires::from_str("fcadb")?)?, 3);

        Ok(())
    }

    #[test]
    fn segment_solver_map_basic() -> Result<(), Box<dyn std::error::Error>> {
        let segmap = SegmentSolver::create_map("abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg")?;

        assert_eq!(segmap.decode(&wire::Wires::from_str("cf")?)?, 1);
        assert_eq!(segmap.decode(&wire::Wires::from_str("bcdf")?)?, 4);
        assert_eq!(segmap.decode(&wire::Wires::from_str("acf")?)?, 7);
        assert_eq!(segmap.decode(&wire::Wires::from_str("abcdefg")?)?, 8);

        assert_eq!(segmap.decode(&wire::Wires::from_str("abdefg")?)?, 6);
        assert_eq!(segmap.decode(&wire::Wires::from_str("abcdfg")?)?, 9);
        assert_eq!(segmap.decode(&wire::Wires::from_str("acdfg")?)?, 3);

        assert_eq!(segmap.decode(&wire::Wires::from_str("abcefg")?)?, 0);
        assert_eq!(segmap.decode(&wire::Wires::from_str("abdfg")?)?, 5);
        assert_eq!(segmap.decode(&wire::Wires::from_str("acdeg")?)?, 2);

        Ok(())
    }

    #[test]
    fn segment_solver_map_example() -> Result<(), Box<dyn std::error::Error>> {
        let segmap = SegmentSolver::create_map("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab")?;

        assert_eq!(segmap.decode(&wire::Wires::from_str("acedgfb")?)?, 8);
        assert_eq!(segmap.decode(&wire::Wires::from_str("cdfbe")?)?, 5);
        assert_eq!(segmap.decode(&wire::Wires::from_str("gcdfa")?)?, 2);
        assert_eq!(segmap.decode(&wire::Wires::from_str("dab")?)?, 7);
        assert_eq!(segmap.decode(&wire::Wires::from_str("cefabd")?)?, 9);
        assert_eq!(segmap.decode(&wire::Wires::from_str("cdfgeb")?)?, 6);
        assert_eq!(segmap.decode(&wire::Wires::from_str("eafb")?)?, 4);
        assert_eq!(segmap.decode(&wire::Wires::from_str("ab")?)?, 1);
        assert_eq!(segmap.decode(&wire::Wires::from_str("cagedb")?)?, 0);
        assert_eq!(segmap.decode(&wire::Wires::from_str("fbcad")?)?, 3);

        Ok(())
    }

    #[test]
    fn solve_segments() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(SegmentSolver::solve_segments("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), Ok(5353));
        assert_eq!(SegmentSolver::solve_segments("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), Ok(8394));
        assert_eq!(SegmentSolver::solve_segments("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"), Ok(9781));

        Ok(())
    }

}
