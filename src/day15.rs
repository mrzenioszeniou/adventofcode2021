use crate::util::a_star;

pub fn solve() -> (usize, usize) {
    let map = parse(std::fs::read_to_string("res/day15.txt").unwrap());

    (part1(&map), part2(&map))
}

fn part1(map: &[Vec<usize>]) -> usize {
    let start = (0, 0);
    let end = (map.len() - 1, map[0].len() - 1);

    let (_, dist) = a_star(
        map,
        start,
        end,
        |_, to| map[to.0][to.1],
        |pos| end.0.abs_diff(pos.0) + end.1.abs_diff(pos.1),
    )
    .expect("No path found");

    dist
}

fn part2(map: &[Vec<usize>]) -> usize {
    let n = map.len();
    let m = map[0].len();

    let mut extended_map = vec![];

    for rep_i in 0..5 {
        for i in 0..n {
            let mut line = vec![];
            for rep_j in 0..5 {
                for j in 0..m {
                    let mut num = map[i % n][j % n] + rep_i + rep_j;

                    if num > 9 {
                        num %= 9;
                    }

                    line.push(num);
                }
            }
            extended_map.push(line);
        }
    }

    part1(&extended_map)
}

fn parse(content: String) -> Vec<Vec<usize>> {
    content
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let map = parse(
            r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
                .to_string(),
        );

        assert_eq!(part1(&map), 40);
        assert_eq!(part2(&map), 315);
    }
}
