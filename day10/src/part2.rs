use crate::parse::{ParseLineError, parse_line};

fn part2_single_score(autocompleted: Vec<char>) -> u64 {
    autocompleted.iter().fold(0, |acc, char| acc * 5 + match char {
                                    ')' => 1,
                                    ']' => 2,
                                    '}' => 3,
                                    '>' => 4,
                                    _ => 0,
                            })
}

fn autocomplete(errstack: Vec<char>) -> Result<Vec<char>, ParseLineError> {
    errstack.iter().rev().map(|c| {
        match c {
            '(' => Ok(')'),
            '{' => Ok('}'),
            '[' => Ok(']'),
            '<' => Ok('>'),
            _ => Err(ParseLineError::IllegalCharacter(*c)),
        }
    }).collect()
}

pub fn get_part2_score(input: &str) -> u64 {
    let mut results: Vec<_> = input.lines().map(|line| {
        if let Err(ParseLineError::StackNonEmpty(st)) = parse_line(line) {
            part2_single_score(autocomplete(st).unwrap())
        } else {
            0
        }}).filter(|val| *val != 0).collect();

    results.sort();

    if let Some(result) = results.get(results.len()/2).to_owned() {
        *result
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part2_single_score() {
        assert_eq!(part2_single_score("}}]])})]".chars().collect()), 288957);
        assert_eq!(part2_single_score(")}>]})".chars().collect()), 5566);
        assert_eq!(part2_single_score("}}>}>))))".chars().collect()), 1480781);
        assert_eq!(part2_single_score("]]}}]}]}>".chars().collect()), 995444);
        assert_eq!(part2_single_score("])}>".chars().collect()), 294);
    }

    #[test]
    fn test_get_part2_score() {
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
        assert_eq!(get_part2_score(input), 288957);
    }
}
