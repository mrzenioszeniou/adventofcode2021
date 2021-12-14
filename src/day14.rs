use std::collections::HashMap;

pub fn solve() -> (usize, usize) {
    let (polymer, pairs) = parse(std::fs::read_to_string("res/day14.txt").unwrap());

    (solver(&polymer, &pairs, 10), solver(&polymer, &pairs, 40))
}

fn solver(polymer: &[char], pairs: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut pair_counts = HashMap::new();
    for i in 0..polymer.len() - 1 {
        pair_counts
            .entry((polymer[i], polymer[i + 1]))
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    let mut element_counts = polymer.iter().fold(HashMap::new(), |mut acc, val| {
        acc.entry(*val).and_modify(|cnt| *cnt += 1).or_insert(1);
        acc
    });

    for _ in 0..steps {
        let mut next = HashMap::new();
        for (pair, count) in pair_counts.into_iter() {
            if let Some(mid) = pairs.get(&pair) {
                next.entry((pair.0, *mid))
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
                next.entry((*mid, pair.1))
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
                element_counts
                    .entry(*mid)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
            } else {
                next.entry(pair)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
            }
        }
        pair_counts = next;
    }

    element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
}

fn parse(content: String) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut split = content.split('\n');

    let polymer = split.next().unwrap().trim().chars().collect();

    assert!(split.next().unwrap().trim().is_empty());

    let mut pairs = HashMap::new();

    for line in split {
        let (left, right, mid) =
            sscanf::scanf!(line.trim(), "{}{} -> {}", char, char, char).unwrap();
        pairs.insert((left, right), mid);
    }

    (polymer, pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (polymer, pairs) = parse(
            r#"NNCB

                CH -> B
                HH -> N
                CB -> H
                NH -> C
                HB -> C
                HC -> B
                HN -> C
                NN -> C
                BH -> H
                NC -> B
                NB -> B
                BN -> B
                BB -> N
                BC -> B
                CC -> N
                CN -> C"#
                .to_string(),
        );

        assert_eq!(solver(&polymer, &pairs, 10), 1588);
        assert_eq!(solver(&polymer, &pairs, 40), 2188189693529);
    }
}
