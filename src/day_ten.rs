use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/10/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let lines: Vec<_> = contents.lines().collect();
    Ok(error_score(lines.as_slice()))
}

pub fn part_two(contents: &str) -> Result<usize> {
    let lines: Vec<_> = contents.lines().collect();
    Ok(completion_score(lines.as_slice()))
}

#[derive(Debug, PartialEq)]
enum Error {
    Unmatched(char),
    Unknown,
}

fn error_score(lines: &[&str]) -> usize {
    check_delimiters(lines)
        .iter()
        .map(|res| match res {
            Ok(_) => 0,
            Err(Error::Unmatched(')')) => 3,
            Err(Error::Unmatched(']')) => 57,
            Err(Error::Unmatched('}')) => 1197,
            Err(Error::Unmatched('>')) => 25137,
            _ => panic!("Unexpected error"),
        })
        .sum()
}

fn completion_score(lines: &[&str]) -> usize {
    let foo = check_delimiters(lines);

    let lines: Vec<_> = foo
        .iter()
        .flatten() // throw away errors
        .collect();
    let mut scores: Vec<_> = autocomplete_lines(lines.as_slice())
        .iter()
        .map(|line| {
            line.chars().fold(0, |score, character| {
                (score * 5)
                    + if character == ')' {
                        1
                    } else if character == ']' {
                        2
                    } else if character == '}' {
                        3
                    } else {
                        4
                    }
            })
        })
        .collect();
    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}

// lines must be filtered to have no unmatched closing braces
fn autocomplete_lines(lines: &[&&str]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let mut opened = line.chars().fold(Vec::new(), |mut opened, character| {
                if "([{<".contains(character) {
                    opened.push(character);
                    opened
                } else if let Some(_) = opened.last() {
                    opened.pop(); // we trust there are no unmatched braces
                    opened
                } else {
                    opened
                }
            });
            opened.reverse();

            opened.iter().map(|c| matching(*c)).collect()
        })
        .collect()
}

fn check_delimiters<'a>(lines: &'a [&str]) -> Vec<std::result::Result<&'a str, Error>> {
    lines
        .iter()
        .map(|line| {
            let line_res = line.chars().try_fold(Vec::new(), |mut opened, character| {
                if "([{<".contains(character) {
                    opened.push(character);
                } else if let Some(last) = opened.last() {
                    if matching(*last) == character {
                        opened.pop();
                    } else {
                        return Err(Error::Unmatched(character));
                    }
                } else {
                    return Err(Error::Unknown);
                }
                Ok(opened)
            });
            line_res.map(|_| *line)
        })
        .collect()
}

fn matching(open: char) -> char {
    if open == '(' {
        ')'
    } else if open == '[' {
        ']'
    } else if open == '{' {
        '}'
    } else if open == '<' {
        '>'
    } else {
        panic!("invalid input")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_score() {
        let input = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        assert_eq!(288957, completion_score(&input));
    }

    /*
    #[test]
    fn test_autocomplete_lines() {
        let input = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "(((({<>}<{<{<>}{[]{[]{}",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let expected = vec!["}}]])})]", ")}>]})", "}}>}>))))", "]]}}]}]}>", "])}>"];
        assert_eq!(expected, autocomplete_lines(&input));
    }
    */

    #[test]
    fn test_check_delimiters() {
        let input = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let expected = vec![
            Ok("[({(<(())[]>[[{[]{<()<>>"),
            Ok("[(()[<>])]({[<{<<[]>>("),
            Err(Error::Unmatched('}')),
            Ok("(((({<>}<{<{<>}{[]{[]{}"),
            Err(Error::Unmatched(')')),
            Err(Error::Unmatched(']')),
            Ok("{<[[]]>}<{[{[{[]{()[[[]"),
            Err(Error::Unmatched(')')),
            Err(Error::Unmatched('>')),
            Ok("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];
        assert_eq!(expected, check_delimiters(&input));
    }
}
