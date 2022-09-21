use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum CaveGraphError {
    #[error("Parsing failure")]
    ParseError,
    #[error("Missing node")]
    MissingNode,
}

const START_NODE: &str = "start";
const END_NODE: &str = "end";

#[derive(Default, Debug, Clone)]
pub struct CaveGraph(HashMap<String, Vec<String>>);

#[derive(PartialEq, Eq)]
pub enum PathsVariant {
    AllSmallCavesOnce,
    OneSmallCaveTwice,
}

impl CaveGraph {
    fn add_edge(&mut self, from: String, to: String) {
        if let Some(neighbours) = self.0.get_mut(&from) {
            if to != START_NODE && from != END_NODE {
                neighbours.push(to);
            }
        } else {
            if to != START_NODE && from != END_NODE {
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

    fn is_small_cave_visited_twice(candidate: &String, path: &Vec<String>) -> bool {
        if candidate.to_lowercase() != *candidate {
            false
        } else {
            path.contains(candidate)
        }
    }

    fn is_one_small_visited_more_than_twice(path: &Vec<String>) -> bool {

        let mut detected_twice_cave = None;

        for small_cave in path.iter().filter(|node| node.to_lowercase() == **node) {
            match path.iter().filter(|cave| *cave == small_cave).count() {
                1 => {},
                2 if detected_twice_cave == None => { detected_twice_cave = Some(small_cave); },
                2 if detected_twice_cave == Some(small_cave) => {},
                _ => { return true; }
            }
        }
        false
    }

    pub fn paths(&self, variant: PathsVariant) -> Result<HashSet<Vec<String>>, CaveGraphError> {
        let mut results = HashSet::new();
        let mut fifo: VecDeque<Vec<String>> = VecDeque::from([vec![START_NODE.to_string()]]);

        while let Some(path) = fifo.pop_back() {
            let last_node = path.last().ok_or(CaveGraphError::MissingNode)?;
            let next_nodes = self.0.get(last_node).ok_or(CaveGraphError::MissingNode)?;
            for next_node in next_nodes.iter() {
                if variant == PathsVariant::AllSmallCavesOnce && Self::is_small_cave_visited_twice(next_node, &path) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(next_node.to_string());

                if variant == PathsVariant::OneSmallCaveTwice && Self::is_one_small_visited_more_than_twice(&new_path) {
                    continue;
                }

                if next_node == END_NODE {
                    results.insert(new_path);
                } else {
                    fifo.push_front(new_path);
                }
            }
        }

        Ok(results)
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
    fn test_find_all_paths_start_to_end() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-end")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce), Ok(HashSet::from([vec!["start", "end"].into_iter().map(str::to_string).collect()])));

        Ok(())
    }

    #[test]
    fn test_find_all_paths_single_cave() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("end-A")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce), Ok(HashSet::from([vec!["start", "A", "end"].into_iter().map(str::to_string).collect()])));

        Ok(())
    }

    #[test]
    fn test_find_all_paths_two_caves() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("end-A")?;

        graph.add_entry("start-B")?;
        graph.add_entry("end-B")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce), Ok(HashSet::from([vec!["start", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "B", "end"].into_iter().map(str::to_string).collect()])));

        Ok(())
    }

    #[test]
    fn test_find_all_paths_two_caves_connected() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("start-A")?;
        graph.add_entry("A-B")?;
        graph.add_entry("end-A")?;
        graph.add_entry("end-B")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce), Ok(HashSet::from([vec!["start", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "B", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "B", "A", "end"].into_iter().map(str::to_string).collect()])));

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

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce), Ok(HashSet::from([vec!["start", "A", "b", "A", "c", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "b", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "b", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "c", "A", "b", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "c", "A", "b", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "c", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "b", "A", "c", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "b", "A", "end"].into_iter().map(str::to_string).collect(),
                                                 vec!["start", "b", "end"].into_iter().map(str::to_string).collect()])));

        assert_eq!(graph.paths(PathsVariant::OneSmallCaveTwice)?.len(), 36);

        Ok(())
    }

    #[test]
    fn test_find_all_paths_2() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("dc-end")?;
        graph.add_entry("HN-start")?;
        graph.add_entry("start-kj")?;
        graph.add_entry("dc-start")?;
        graph.add_entry("dc-HN")?;
        graph.add_entry("LN-dc")?;
        graph.add_entry("HN-end")?;
        graph.add_entry("kj-sa")?;
        graph.add_entry("kj-HN")?;
        graph.add_entry("kj-dc")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce)?.len(), 19);

        Ok(())
    }

    #[test]
    fn test_find_all_paths_3() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();
        graph.add_entry("fs-end")?;
        graph.add_entry("he-DX")?;
        graph.add_entry("fs-he")?;
        graph.add_entry("start-DX")?;
        graph.add_entry("pj-DX")?;
        graph.add_entry("end-zg")?;
        graph.add_entry("zg-sl")?;
        graph.add_entry("zg-pj")?;
        graph.add_entry("pj-he")?;
        graph.add_entry("RW-he")?;
        graph.add_entry("fs-DX")?;
        graph.add_entry("pj-RW")?;
        graph.add_entry("zg-RW")?;
        graph.add_entry("start-pj")?;
        graph.add_entry("he-WI")?;
        graph.add_entry("zg-he")?;
        graph.add_entry("pj-fs")?;
        graph.add_entry("start-RW")?;

        assert_eq!(graph.paths(PathsVariant::AllSmallCavesOnce)?.len(), 226);

        Ok(())
    }
}

