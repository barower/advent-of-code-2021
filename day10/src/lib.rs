#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseLineError {
    StackNonEmpty(Vec<char>),
    IllegalCharacter(char),
    WrongParenthesis(char),
}

fn is_opening(opening: char) -> bool {
    match opening {
        '{' | '(' | '[' | '<' => true,
        _ => false,
    }
}

fn is_ending(ending: char) -> bool {
    match ending {
        '}' | ')' | ']' | '>' => true,
        _ => false,
    }
}

fn are_matching(opening: char, ending: char) -> bool {
    match (opening, ending) {
        ('{', '}') | ('(', ')') | ('[', ']') | ('<', '>') => true,
        _ => false,
    }
}

fn part1_single_score(ending: char) -> u64 {
    match ending {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part2_single_score(autocompleted: Vec<char>) -> u64 {
    autocompleted.iter().fold(0, |acc, char| acc * 5 + match char {
                                    ')' => 1,
                                    ']' => 2,
                                    '}' => 3,
                                    '>' => 4,
                                    _ => 0,
                            })
}

fn parse_line(line: &str) -> Result<(), ParseLineError> {
    let mut stack: Vec<char> = Vec::with_capacity(line.len()/2);

    for (i, c) in line.chars().enumerate() {
        let stack_top = stack.last().cloned();
        match (stack_top, c) {
            (None, opening) if is_opening(opening) => { stack.push(c); }
            (Some(opening), ending) if are_matching(opening, ending) => { stack.pop(); },
            (Some(opening0), opening1) if is_opening(opening0) && is_opening(opening1) => { stack.push(c); },
            (Some(opening), ending) if is_opening(opening) && is_ending(ending) && !are_matching(opening, ending) => { return Err(ParseLineError::WrongParenthesis(c)); },
            _ => { return Err(ParseLineError::IllegalCharacter(c)); },
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(ParseLineError::StackNonEmpty(stack))
    }
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

pub fn get_part1_score(input: &str) -> u64 {
    input.lines().map(|line| {
        if let Err(ParseLineError::WrongParenthesis(c)) = parse_line(line) {
            part1_single_score(c)
        } else {
            0
        }}).sum()
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
    fn test_ok_lines() {
        assert_eq!(parse_line("()"), Ok(()));
        assert_eq!(parse_line("<><><><>"), Ok(()));
        assert_eq!(parse_line("{()}"), Ok(()));
        assert_eq!(parse_line("<([{}])>"), Ok(()));
        assert_eq!(parse_line("[<>({}){}[([])<>]]"), Ok(()));
        assert_eq!(parse_line("(((((((((())))))))))"), Ok(()));
    }

    #[test]
    fn test_broken_lines() {
        assert_eq!(parse_line("(]"), Err(ParseLineError::WrongParenthesis(']')));
        assert_eq!(parse_line("{()()()>"), Err(ParseLineError::WrongParenthesis('>')));
        assert_eq!(parse_line("(((()))}"), Err(ParseLineError::WrongParenthesis('}')));
        assert_eq!(parse_line("<([]){()}[{}])"), Err(ParseLineError::WrongParenthesis(')')));
    }

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
