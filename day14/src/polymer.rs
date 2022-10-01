use crate::rules::Rules;
use std::collections::HashMap;

pub trait Polymer {
    fn step(self, rules: &Rules) -> Self;
    fn char_occurencies(&self) -> HashMap<char, usize>;
}

pub fn sorted_char_occurences(polymer: impl Polymer) -> Vec<(char, usize)> {
    let countmap = polymer.char_occurencies();

    let mut retvec: Vec<(char, usize)> = countmap.into_iter().collect();
    retvec.sort_unstable_by_key(|&(_, s)| s);

    retvec
}
