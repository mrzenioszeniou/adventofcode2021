use std::collections::HashSet;

use sscanf::scanf;

pub fn solve() -> (usize, String) {
    let (dots, folds) = parse(std::fs::read_to_string("res/day13.txt").unwrap());

    (part1(&dots, &folds[0]), part2(dots, &folds))
}

fn part1(dots: &HashSet<(usize, usize)>, fold: &Fold) -> usize {
    fold.fold(dots).len()
}

fn part2(mut dots: HashSet<(usize, usize)>, folds: &[Fold]) -> String {
    for fold in folds {
        dots = fold.fold(&dots);
    }

    let max_x = dots.iter().map(|(x, _)| x).max().unwrap();
    let max_y = dots.iter().map(|(_, y)| y).max().unwrap();

    let mut ret = String::from("\n");

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if dots.contains(&(x, y)) {
                ret.push('▓');
            } else {
                ret.push(' ');
            }
        }
        ret.push('\n');
    }

    ret
}

enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

impl Fold {
    pub fn fold(&self, dots: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        dots.iter()
            .map(|dot| match self {
                Fold::Vertical(x) if dot.0 > *x => (2 * x - dot.0, dot.1),
                Fold::Horizontal(y) if dot.1 > *y => (dot.0, 2 * y - dot.1),
                _ => *dot,
            })
            .collect()
    }
}

fn parse(s: String) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let mut dots = HashSet::new();

    let mut split = s.split('\n');

    for line in split.by_ref() {
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let dot = scanf!(line, "{},{}", usize, usize).unwrap();

        dots.insert(dot);
    }

    let mut folds = vec![];

    for line in split {
        let line = line.trim();

        let (dim, num) = scanf!(line, "fold along {}={}", char, usize).unwrap();

        folds.push(match dim {
            'x' => Fold::Vertical(num),
            'y' => Fold::Horizontal(num),
            _ => panic!("Unexpected dimension character '{}'", dim),
        });
    }

    (dots, folds)
}

#[cfg(test)]
mod tests {
    use crate::day13::part1;

    use super::parse;

    #[test]
    fn example() {
        let (dots, folds) = parse(
            r#"6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5"#
                .to_string(),
        );

        assert_eq!(part1(&dots, &folds[0]), 17)
    }
}
