use crate::parse::{ParseLineError, parse_line};

fn part1_single_score(ending: char) -> u64 {
    match ending {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn get_part1_score(input: &str) -> u64 {
    input.lines().map(|line| {
        if let Err(ParseLineError::WrongParenthesis(c)) = parse_line(line) {
            part1_single_score(c)
        } else {
            0
        }}).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part1_score() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(get_part1_score(input), 26397);
    }

}
