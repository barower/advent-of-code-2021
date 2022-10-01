use crate::polymer::*;
use crate::rules::Rules;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct NaivePolymer(Vec<char>);

impl NaivePolymer {
    pub fn new(input: &str) -> Self {
        NaivePolymer(input.chars().collect())
    }
}

impl Polymer for NaivePolymer {
    fn step(self, rules: &Rules) -> Self {
        let mut retvec = Vec::with_capacity(self.0.len() * 2 - 1);

        for window in self.0.windows(2) {
            retvec.push(window[0]);
            rules
                .find_insertion(&(window[0], window[1]))
                .map(|c| retvec.push(*c));
        }

        self.0.last().map(|c| retvec.push(*c));

        NaivePolymer(retvec)
    }

    fn char_occurencies(&self) -> HashMap<char, usize> {
        let mut countmap: HashMap<char, usize> = HashMap::new();

        for c in self.0.iter() {
            if let Some(count) = countmap.get_mut(c) {
                *count += 1;
            } else {
                countmap.insert(*c, 1);
            }
        }

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

        let polymer = NaivePolymer::new("NNCB");

        let polymer1 = polymer.step(&rules);
        assert_eq!(polymer1, NaivePolymer::new("NCNBCHB"));

        let polymer2 = polymer1.step(&rules);
        assert_eq!(polymer2, NaivePolymer::new("NBCCNBBBCBHCB"));

        let polymer3 = polymer2.step(&rules);
        assert_eq!(polymer3, NaivePolymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB"));

        let polymer4 = polymer3.step(&rules);
        assert_eq!(
            polymer4,
            NaivePolymer::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );

        Ok(())
    }

    #[test]
    fn polymer_char_count() -> Result<(), Box<dyn std::error::Error>> {
        let rules = example_rules()?;

        let mut polymer = NaivePolymer::new("NNCB");

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
