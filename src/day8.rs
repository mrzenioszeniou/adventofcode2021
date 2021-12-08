use std::collections::{HashMap, HashSet};

pub fn solve() -> (usize, usize) {
    let input = parse();

    (part1(input.iter().map(|(_, v)| v).collect()), part2(input))
}

fn part1(values: Vec<&Vec<String>>) -> usize {
    values
        .iter()
        .map(|value| {
            value
                .iter()
                .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn part2(displays: Vec<(Vec<String>, Vec<String>)>) -> usize {
    displays
        .into_iter()
        .map(|display| segment_reader(display.0, display.1))
        .sum()
}

/// For the given number returns the corresponding "ON" segments
fn num_to_segments(num: u8) -> HashSet<char> {
    match num {
        0 => HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']),
        1 => HashSet::from(['c', 'f']),
        2 => HashSet::from(['a', 'c', 'd', 'e', 'g']),
        3 => HashSet::from(['a', 'c', 'd', 'f', 'g']),
        4 => HashSet::from(['b', 'c', 'd', 'f']),
        5 => HashSet::from(['a', 'b', 'd', 'f', 'g']),
        6 => HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']),
        7 => HashSet::from(['a', 'c', 'f']),
        8 => HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        9 => HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']),
        _ => panic!("Can't turn '{}' into segment set", num),
    }
}

/// For the given set of "ON" segments returns the corresponding number
fn segments_to_num(segments: HashSet<char>) -> u8 {
    for num in 0..10 {
        if segments == num_to_segments(num) {
            return num;
        }
    }

    panic!("Could not translate {:?} to a number", segments);
}

/// Takes a map of characters, translates the provided digits, and turns them into a number
fn translate(mapping: &HashMap<char, char>, digits: Vec<String>) -> usize {
    digits
        .into_iter()
        .map(|digit| segments_to_num(digit.chars().map(|c| *mapping.get(&c).unwrap()).collect()))
        .rev()
        .enumerate()
        .map(|(pos, digit)| digit as usize * 10_usize.pow(pos as u32))
        .sum()
}

/// Unscrambles the test digits and translates the value digits to a number
fn segment_reader(test_digits: Vec<String>, value_digits: Vec<String>) -> usize {
    let digit_possibilities = test_digits
        .iter()
        .map(|digit| match digit.len() {
            2 => (digit, HashSet::from([1])),
            3 => (digit, HashSet::from([7])),
            4 => (digit, HashSet::from([4])),
            5 => (digit, HashSet::from([2, 3, 5])),
            6 => (digit, HashSet::from([0, 6, 9])),
            7 => (digit, HashSet::from([8])),
            _ => panic!(
                "Found scrambled digit with unexpected amount of characters:{}",
                digit
            ),
        })
        .collect();

    let digit_combos = crate::util::product(digit_possibilities, true);
    for digit_combo in digit_combos.into_iter() {
        let mut segment_possiblities: HashMap<char, HashSet<char>> =
            ('a'..='g').map(|c| (c, ('a'..='g').collect())).collect();

        for (digit, number) in digit_combo.iter() {
            // If the digit indeed
            let possible_segments = num_to_segments(*number);
            for segment in digit.chars() {
                if let Some(possibilities) = segment_possiblities.get_mut(&segment) {
                    possibilities.retain(|c| possible_segments.contains(c));
                }
            }
        }

        let mappings = crate::util::product(segment_possiblities, true);

        if mappings.len() == 1 {
            return translate(&mappings[0], value_digits);
        } else {
            assert!(
                mappings.is_empty(),
                "We should have either one or no solutions at all"
            );
        }
    }

    panic!("No solution found for {:?}", test_digits);
}

fn parse() -> Vec<(Vec<String>, Vec<String>)> {
    std::fs::read_to_string("res/day8.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            let mut split = line.split('|');

            let segment_digits: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            let value_digits: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            assert_eq!(segment_digits.len(), 10);
            assert_eq!(value_digits.len(), 4);

            (segment_digits, value_digits)
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            segment_reader(
                vec![
                    String::from("acedgfb"),
                    String::from("cdfbe"),
                    String::from("gcdfa"),
                    String::from("fbcad"),
                    String::from("dab"),
                    String::from("cefabd"),
                    String::from("cdfgeb"),
                    String::from("eafb"),
                    String::from("cagedb"),
                    String::from("ab")
                ],
                vec![
                    String::from("cdfeb"),
                    String::from("fcadb"),
                    String::from("cdfeb"),
                    String::from("cdbaf")
                ]
            ),
            5353
        );
    }
}
