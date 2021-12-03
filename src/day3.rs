use std::ops::Not;

pub fn solve() -> (usize, usize) {
    let report = parse();

    (part1(&report), 42)
}

fn part1(report: &[Vec<usize>]) -> usize {
    let n_bits = report[0].len();
    let mut num = 0;

    for j in 0..n_bits {
        let count = report.iter().filter(|line| line[j] == 1).count();

        if count > report.len() / 2 {
            num += 2_usize.pow((n_bits - 1 - j) as u32);
        }
    }

    num * (num.not() % 2_usize.pow(n_bits as u32))
}

fn parse() -> Vec<Vec<usize>> {
    std::fs::read_to_string("res/day3.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}
