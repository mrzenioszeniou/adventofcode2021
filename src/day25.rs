use std::collections::HashSet;

pub fn solve() -> (usize, usize) {
    let input = parse(&std::fs::read_to_string("res/day25.txt").unwrap());

    (part1(input), 42)
}

type Initial = (
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    usize,
    usize,
);

/// (East-Facing, South-Facing, Height, Width)
fn parse(from: &str) -> Initial {
    let mut east = HashSet::new();
    let mut south = HashSet::new();

    let mut n = 0;
    let mut m = 0;

    for (i, line) in from.split('\n').enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '>' => {
                    east.insert((i, j));
                }
                'v' => {
                    south.insert((i, j));
                }
                '.' => {}
                _ => panic!("Invalid input character '{}'", c),
            }
            m = j + 1;
        }
        n = i + 1;
    }

    (east, south, n, m)
}

fn part1((mut east, mut south, n, m): Initial) -> usize {
    for i in 0.. {
        let mut change = false;

        let mut next_east = HashSet::new();

        for cucumba in east.iter() {
            let next = (cucumba.0, (cucumba.1 + 1) % m);

            if east.contains(&next) || south.contains(&next) {
                next_east.insert(*cucumba);
            } else {
                next_east.insert(next);
            }
        }

        if east != next_east {
            change = true;
            east = next_east;
        }

        let mut next_south = HashSet::new();

        for cucumba in south.iter() {
            let next = ((cucumba.0 + 1) % n, cucumba.1);

            if !east.contains(&next) && !south.contains(&next) {
                next_south.insert(next);
            } else {
                next_south.insert(*cucumba);
            }
        }

        if south != next_south {
            change = true;
            south = next_south;
        }

        if !change {
            return i + 1;
        } else {
            println!("{}", i);
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn examples() {
        let input = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

        let initial = parse(input);

        assert_eq!(part1(initial), 58);
    }
}
