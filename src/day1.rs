use std::{fs::File, io::Read};

pub fn solve() -> (usize, usize) {
    let input = parse();

    (part1(&input), part2(&input))
}

fn part1(depths: &[usize]) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(curr, next)| next > curr)
        .count()
}

fn part2(depths: &[usize]) -> usize {
    let mut prev = usize::MAX;
    let mut cnt = 0;

    for i in 0..depths.len() - 2 {
        let curr = depths[i] + depths[i + 1] + depths[i + 2];

        if curr > prev {
            cnt += 1;
        }

        prev = curr;
    }

    cnt
}

fn parse() -> Vec<usize> {
    let mut content = String::new();

    File::open("res/day1.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    content
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("Could not parse as integer"))
        .collect()
}
