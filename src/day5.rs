use std::collections::HashMap;

use crate::util::gcd;

pub fn solve() -> (usize, usize) {
    let lines = parse();

    (solver(&lines, false), solver(&lines, true))
}

pub fn solver(lines: &[Line], diagonals: bool) -> usize {
    let mut grid: HashMap<Point, usize> = HashMap::new();

    for (from, to) in lines {
        // Skip diagonal lines
        if !diagonals && from.0 != to.0 && from.1 != to.1 {
            continue;
        }

        let step_gcd = gcd(to.0 - from.0, to.1 - from.1);

        let step = ((to.0 - from.0) / step_gcd, (to.1 - from.1) / step_gcd);

        let mut curr = *from;

        grid.entry(curr).and_modify(|n| *n += 1).or_insert(1);

        while curr != *to {
            curr = (curr.0 + step.0, curr.1 + step.1);
            grid.entry(curr).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    grid.values().filter(|cnt| **cnt > 1).count()
}

type Point = (isize, isize);
type Line = (Point, Point);

pub fn parse() -> Vec<Line> {
    let content = std::fs::read_to_string("res/day5.txt").unwrap();

    content
        .split('\n')
        .map(|line| {
            let mut points = line.split_whitespace();

            let mut from = points.next().unwrap().split(',');
            let from_j = from.next().unwrap().parse().unwrap();
            let from_i = from.next().unwrap().parse().unwrap();

            let mut to = points.next().unwrap().split(',');
            let to_j = to.next().unwrap().parse().unwrap();
            let to_i = to.next().unwrap().parse().unwrap();

            ((from_i, from_j), (to_i, to_j))
        })
        .collect()
}
