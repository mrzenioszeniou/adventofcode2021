use std::collections::{BTreeSet, HashSet};

use crate::dir::Direction;

pub fn solve() -> (u32, usize) {
    let heightmap = parse();

    (part1(&heightmap), part2(&heightmap))
}

fn part1(heightmap: &[Vec<u32>]) -> u32 {
    find_lowpoints(heightmap)
        .iter()
        .map(|(i, j)| heightmap[*i][*j] + 1)
        .sum()
}

fn part2(heightmap: &[Vec<u32>]) -> usize {
    let n = heightmap.len();
    let m = heightmap[0].len();

    let mut basins: Vec<usize> = find_lowpoints(heightmap)
        .into_iter()
        .map(|low| {
            let mut visited = HashSet::new();
            let mut to_visit = BTreeSet::from([low]);

            while let Some(pos) = to_visit.pop_first() {
                neighbours(pos, n, m)
                    .into_iter()
                    .filter(|n| !visited.contains(n) && heightmap[n.0][n.1] != 9)
                    .for_each(|n| {
                        to_visit.insert(n);
                    });
                visited.insert(pos);
            }

            visited.len()
        })
        .collect();

    assert!(basins.len() >= 3);

    basins.sort_unstable();

    basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
}

fn neighbours(pos: (usize, usize), n: usize, m: usize) -> Vec<(usize, usize)> {
    let pos = (pos.0 as isize, pos.1 as isize);
    let n = n as isize;
    let m = m as isize;

    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .map(|dir| {
        let step = dir.forward();
        (pos.0 + step.0, pos.1 + step.1)
    })
    .filter(|&(i, j)| i >= 0 && i < n && j >= 0 && j < m)
    .map(|(i, j)| (i as usize, j as usize))
    .collect()
}

fn find_lowpoints(heightmap: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            let height = heightmap[i][j];

            if neighbours((i, j), heightmap.len(), heightmap[i].len())
                .into_iter()
                .all(|(i, j)| heightmap[i][j] > height)
            {
                ret.insert((i, j));
            }
        }
    }

    ret
}

fn parse() -> Vec<Vec<u32>> {
    std::fs::read_to_string("res/day9.txt")
        .unwrap()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day9::part2;

    #[test]
    fn example() {
        let heightmap = [
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(part2(&heightmap), 1134);
    }
}
