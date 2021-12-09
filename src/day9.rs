use crate::dir::neighbours_bounded;
use std::collections::{BTreeSet, HashSet};

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
                neighbours_bounded(pos, n, m)
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

fn find_lowpoints(heightmap: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();

    let n = heightmap.len();
    let m = heightmap[0].len();

    for i in 0..n {
        for j in 0..m {
            let height = heightmap[i][j];

            if neighbours_bounded((i, j), n, m)
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
