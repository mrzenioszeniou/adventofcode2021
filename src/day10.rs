use std::collections::HashMap;

pub fn solve() -> (usize, usize) {
    let lines = parse();

    let (filtered, score) = filter_corrupted(&lines);

    (score, autocomplete(&filtered))
}

fn filter_corrupted(lines: &[String]) -> (Vec<String>, usize) {
    let pairs = bracket_pairs();
    let points = syntax_points();

    let mut filtered = vec![];
    let mut score = 0;
    'outer: for line in lines.iter() {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(opener) = stack.last() {
                        if *pairs.get(opener).unwrap() == c {
                            stack.pop();
                        } else {
                            score += points.get(&c).unwrap();
                            continue 'outer;
                        }
                    }
                }
                _ => panic!("Dafuq is '{}'?", c),
            }
        }

        assert!(!stack.is_empty());

        filtered.push(line.clone());
    }

    (filtered, score)
}

fn autocomplete(lines: &[String]) -> usize {
    let pairs = bracket_pairs();
    let points = autocomplete_points();

    let mut scores = vec![];
    for line in lines {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let opener = stack.pop().unwrap();
                    let closer = pairs.get(&opener).unwrap();
                    assert_eq!(c, *closer);
                }
                _ => panic!("Dafuq is '{}'?", c),
            }
        }

        let mut score = 0;
        for opener in stack.into_iter().rev() {
            let closer = pairs.get(&opener).unwrap();
            let points = points.get(closer).unwrap();

            score = score * 5 + points;
        }

        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse() -> Vec<String> {
    std::fs::read_to_string("res/day10.txt")
        .unwrap()
        .split('\n')
        .map(String::from)
        .collect()
}

fn bracket_pairs() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

fn syntax_points() -> HashMap<char, usize> {
    HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)])
}

fn autocomplete_points() -> HashMap<char, usize> {
    HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)])
}

#[cfg(test)]
mod tests {
    use crate::day10::autocomplete;

    use super::filter_corrupted;

    #[test]
    fn example() {
        let lines = vec![
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ];

        let (filtered, score) = filter_corrupted(&lines);

        assert_eq!(score, 26397);

        assert_eq!(autocomplete(&filtered), 288957);
    }
}
