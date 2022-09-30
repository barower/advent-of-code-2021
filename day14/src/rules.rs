use crate::polymererror::PolymerError;
use std::collections::HashMap;

pub struct Rules(HashMap<(char, char), char>);

impl Rules {
    pub fn new() -> Self {
        Rules(HashMap::new())
    }

    pub fn add(&mut self, entry: &str) -> Result<(), PolymerError> {
        let mut sp = entry.split(" -> ");
        let mut cs = sp.next().ok_or(PolymerError::ParseError)?.chars();

        let first = cs.next().ok_or(PolymerError::ParseError)?;
        let second = cs.next().ok_or(PolymerError::ParseError)?;

        let resolves_to = sp
            .next()
            .ok_or(PolymerError::ParseError)?
            .chars()
            .next()
            .ok_or(PolymerError::ParseError)?;

        self.0.insert((first, second), resolves_to);

        Ok(())
    }

    pub fn find_insertion(&self, input_pair: &(char, char)) -> Option<&char> {
        self.0.get(input_pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules() -> Result<(), Box<dyn std::error::Error>> {
        let mut rules = Rules::new();
        rules.add("AA -> B")?;
        assert_eq!(rules.find_insertion(&('A', 'A')), Some(&'B'));
        assert_eq!(rules.find_insertion(&('A', 'C')), None);
        Ok(())
    }
}
