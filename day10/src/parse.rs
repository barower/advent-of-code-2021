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


pub fn parse_line(line: &str) -> Result<(), ParseLineError> {
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
}
