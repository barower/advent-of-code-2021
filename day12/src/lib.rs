use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CaveGraphError {
    #[error("Parsing failure")]
    ParseError,
}

#[derive(Default)]
pub struct CaveGraph(HashMap<String, Vec<String>>);

impl CaveGraph {
    fn add_edge(&mut self, from: String, to: String) {
        if let Some(neighbours) = self.0.get_mut(&from) {
            if to != "start" && from != "end" {
                neighbours.push(to);
            }
        } else {
            if to != "start" && from != "end" {
                self.0.insert(from, vec![to]);
            }
        }
    }

    pub fn add_entry(&mut self, line: &str) -> Result<(), CaveGraphError> {
        let (first, second) = {
            let mut split = line.split('-');
            (split.next().ok_or(CaveGraphError::ParseError)?, split.next().ok_or(CaveGraphError::ParseError)?)
        };

        self.add_edge(first.to_string(), second.to_string());
        self.add_edge(second.to_string(), first.to_string());

        Ok(())
    }

    pub fn paths(self) -> HashSet<Vec<&'static str>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_entry() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("A-c")?;
        graph.add_entry("A-b")?;

        assert_eq!(graph.0.get("A"), Some(&vec!["c".to_string(), "b".to_string()]));
        assert_eq!(graph.0.get("c"), Some(&vec!["A".to_string()]));
        assert_eq!(graph.0.get("b"), Some(&vec!["A".to_string()]));

        Ok(())
    }

    #[test]
    fn add_entry_with_start() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("start-b")?;
        graph.add_entry("A-c")?;
        graph.add_entry("A-b")?;

        assert_eq!(graph.0.get("start"), Some(&vec!["A".to_string(), "b".to_string()]));
        assert_eq!(graph.0.get("A"), Some(&vec!["c".to_string(), "b".to_string()]));
        assert_eq!(graph.0.get("c"), Some(&vec!["A".to_string()]));
        assert_eq!(graph.0.get("b"), Some(&vec!["A".to_string()]));

        Ok(())
    }

    #[test]
    fn add_entry_with_start_and_end() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("start-b")?;
        graph.add_entry("A-c")?;
        graph.add_entry("A-b")?;
        graph.add_entry("A-end")?;
        graph.add_entry("b-end")?;

        assert_eq!(graph.0.get("start"), Some(&vec!["A".to_string(), "b".to_string()]));
        assert_eq!(graph.0.get("A"), Some(&vec!["c".to_string(), "b".to_string(), "end".to_string()]));
        assert_eq!(graph.0.get("c"), Some(&vec!["A".to_string()]));
        assert_eq!(graph.0.get("b"), Some(&vec!["A".to_string(), "end".to_string()]));
        assert_eq!(graph.0.get("end"), None);

        Ok(())
    }

    #[test]
    fn test_find_all_paths() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("start-b")?;
        graph.add_entry("A-c")?;
        graph.add_entry("A-b")?;
        graph.add_entry("b-d")?;
        graph.add_entry("A-end")?;
        graph.add_entry("b-end")?;

        assert_eq!(graph.paths(), HashSet::from([vec!["start", "A", "b", "A", "c", "A", "end"],
                                                 vec!["start", "A", "b", "A", "end"],
                                                 vec!["start", "A", "b", "end"],
                                                 vec!["start", "A", "c", "A", "b", "A", "end"],
                                                 vec!["start", "A", "c", "A", "b", "end"],
                                                 vec!["start", "A", "c", "A", "end"],
                                                 vec!["start", "A", "end"],
                                                 vec!["start", "b", "A", "end"],
                                                 vec!["start", "b", "end"]]));

        Ok(())
    }
}

