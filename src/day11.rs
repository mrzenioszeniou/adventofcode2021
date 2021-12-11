use std::collections::HashSet;

use crate::dir::{neighbours_bounded, neighbours_bounded_diagonal};

pub fn solve() -> (usize, usize) {
    let school = parse();

    (part1(&school), part2(&school))
}

fn part1(school: &[Vec<u32>]) -> usize {
    let mut school = school.to_vec();

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += simulate(&mut school);
    }

    flashes
}

fn part2(school: &[Vec<u32>]) -> usize {
    let mut school = school.to_vec();

    let cnt = school.len() * school[0].len();

    for i in 1.. {
        if simulate(&mut school) == cnt {
            return i;
        }
    }

    unreachable!();
}

fn simulate(school: &mut [Vec<u32>]) -> usize {
    let n = school.len();
    let m = school[0].len();

    school
        .iter_mut()
        .for_each(|line| line.iter_mut().for_each(|o| *o += 1));

    let mut to_flash = HashSet::new();
    loop {
        let mut changed = false;

        for i in 0..n {
            for j in 0..m {
                if school[i][j] > 9 && to_flash.insert((i, j)) {
                    neighbours_bounded((i, j), n, m)
                        .union(&neighbours_bounded_diagonal((i, j), n, m))
                        .for_each(|n| school[n.0][n.1] += 1);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut cnt = 0;

    for (i, j) in to_flash.into_iter() {
        school[i][j] = 0;
        cnt += 1;
    }

    cnt
}

fn parse() -> Vec<Vec<u32>> {
    std::fs::read_to_string("res/day11.txt")
        .unwrap()
        .split('\n')
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let school = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(part1(&school), 1656);
        assert_eq!(part2(&school), 195);
    }
}
