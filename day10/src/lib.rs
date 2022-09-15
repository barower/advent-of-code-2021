#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseLineError {
    StackNonEmpty,
    IllegalCharacter(usize),
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

fn score(ending: char) -> u64 {
    match ending {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
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
            _ => { return Err(ParseLineError::IllegalCharacter(i)); },
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(ParseLineError::StackNonEmpty)
    }
}

pub fn get_total_score(input: &str) -> u64 {
    input.lines().map(|line| {
        if let Err(ParseLineError::WrongParenthesis(c)) = parse_line(line) {
            score(c)
        } else {
            0
        }}).sum()
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
    fn get_score() {
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
        assert_eq!(get_total_score(input), 26397);
    }
}
