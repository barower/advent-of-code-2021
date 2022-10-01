use crate::polymer::*;
use crate::rules::Rules;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct OptimalPolymer {
    pair_count: HashMap<(char, char), usize>,
    last_char: char,
}

impl OptimalPolymer {
    pub fn new(input: &str) -> Self {
        let mut pair_count = HashMap::new();

        for window in input.chars().collect::<Vec<char>>().windows(2) {
            *pair_count.entry((window[0], window[1])).or_insert(0) += 1
        }

        OptimalPolymer {
            pair_count,
            last_char: input.chars().last().unwrap(),
        }
    }
}

impl Polymer for OptimalPolymer {
    fn step(self, rules: &Rules) -> Self {
        let mut new_pair_count: HashMap<(char, char), usize> = HashMap::new();

        for ((f, s), count) in self.pair_count.iter() {
            if let Some(x) = rules.find_insertion(&(*f, *s)) {
                *new_pair_count.entry((*f, *x)).or_insert(0) += count;
                *new_pair_count.entry((*x, *s)).or_insert(0) += count;
            } else {
                *new_pair_count.entry((*f, *s)).or_insert(0) += count;
            }
        }

        OptimalPolymer {
            pair_count: new_pair_count,
            last_char: self.last_char,
        }
    }

    fn char_occurencies(&self) -> HashMap<char, usize> {
        let mut countmap: HashMap<char, usize> = HashMap::new();

        for ((f, _), count) in self.pair_count.iter() {
            *countmap.entry(*f).or_insert(0) += count;
        }

        *countmap.entry(self.last_char).or_insert(0) += 1;

        countmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_rules() -> Result<Rules, Box<dyn std::error::Error>> {
        let mut rules = Rules::new();
        rules.add("CH -> B")?;
        rules.add("HH -> N")?;
        rules.add("CB -> H")?;
        rules.add("NH -> C")?;
        rules.add("HB -> C")?;
        rules.add("HC -> B")?;
        rules.add("HN -> C")?;
        rules.add("NN -> C")?;
        rules.add("BH -> H")?;
        rules.add("NC -> B")?;
        rules.add("NB -> B")?;
        rules.add("BN -> B")?;
        rules.add("BB -> N")?;
        rules.add("BC -> B")?;
        rules.add("CC -> N")?;
        rules.add("CN -> C")?;
        Ok(rules)
    }

    #[test]
    fn polymer_step() -> Result<(), Box<dyn std::error::Error>> {
        let rules = example_rules()?;

        let polymer = OptimalPolymer::new("NNCB");

        let polymer1 = polymer.step(&rules);
        assert_eq!(polymer1, OptimalPolymer::new("NCNBCHB"));

        let polymer2 = polymer1.step(&rules);
        assert_eq!(polymer2, OptimalPolymer::new("NBCCNBBBCBHCB"));

        let polymer3 = polymer2.step(&rules);
        assert_eq!(polymer3, OptimalPolymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB"));

        let polymer4 = polymer3.step(&rules);
        assert_eq!(
            polymer4,
            OptimalPolymer::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );

        Ok(())
    }

    #[test]
    fn polymer_char_count() -> Result<(), Box<dyn std::error::Error>> {
        let rules = example_rules()?;

        let mut polymer = OptimalPolymer::new("NNCB");

        for _ in 0..10 {
            polymer = polymer.step(&rules);
        }

        let occurencies = polymer.char_occurencies();

        assert_eq!(occurencies.get(&'B'), Some(&1749));
        assert_eq!(occurencies.get(&'C'), Some(&298));
        assert_eq!(occurencies.get(&'H'), Some(&161));
        assert_eq!(occurencies.get(&'N'), Some(&865));

        let sorted_occurencies = sorted_char_occurences(polymer);
        let (_, most) = sorted_occurencies.last().unwrap();
        let (_, least) = sorted_occurencies.first().unwrap();
        assert_eq!(most - least, 1588);

        Ok(())
    }
}
