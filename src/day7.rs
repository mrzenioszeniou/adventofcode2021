use std::cmp::Ordering;

pub fn solve() -> (isize, isize) {
    let crabs = parse();

    (part1(&crabs), part2(&crabs))
}

fn part1(crabs: &[isize]) -> isize {
    let mut curr = crabs[crabs.len() / 2];
    let mut curr_fuel = crabs.iter().map(|c| (c - curr).abs()).sum();

    loop {
        let left_fuel: isize = crabs.iter().map(|c| (c - (curr - 1)).abs()).sum();
        let right_fuel: isize = crabs.iter().map(|c| (c - (curr + 1)).abs()).sum();

        match (left_fuel.cmp(&curr_fuel), right_fuel.cmp(&curr_fuel)) {
            (Ordering::Equal, Ordering::Equal) | (Ordering::Less, Ordering::Less) => {
                panic!("What do I do?")
            }
            (Ordering::Greater, Ordering::Greater) => return curr_fuel,
            (Ordering::Less, Ordering::Equal)
            | (Ordering::Less, Ordering::Greater)
            | (Ordering::Equal, Ordering::Greater) => {
                curr -= 1;
                curr_fuel = left_fuel;
            }
            (Ordering::Equal, Ordering::Less)
            | (Ordering::Greater, Ordering::Less)
            | (Ordering::Greater, Ordering::Equal) => {
                curr += 1;
                curr_fuel = right_fuel;
            }
        }
    }
}

fn part2(crabs: &[isize]) -> isize {
    let mut curr = crabs[crabs.len() / 2];
    let mut curr_fuel = crabs.iter().map(|c| fuel(*c, curr)).sum();

    loop {
        let left_fuel: isize = crabs.iter().map(|c| fuel(*c, curr - 1)).sum();
        let right_fuel: isize = crabs.iter().map(|c| fuel(*c, curr + 1)).sum();

        match (left_fuel.cmp(&curr_fuel), right_fuel.cmp(&curr_fuel)) {
            (Ordering::Equal, Ordering::Equal) | (Ordering::Less, Ordering::Less) => {
                panic!("What do I do?")
            }
            (Ordering::Greater, Ordering::Greater) => return curr_fuel,
            (Ordering::Less, Ordering::Equal)
            | (Ordering::Less, Ordering::Greater)
            | (Ordering::Equal, Ordering::Greater) => {
                curr -= 1;
                curr_fuel = left_fuel;
            }
            (Ordering::Equal, Ordering::Less)
            | (Ordering::Greater, Ordering::Less)
            | (Ordering::Greater, Ordering::Equal) => {
                curr += 1;
                curr_fuel = right_fuel;
            }
        }
    }
}

fn parse() -> Vec<isize> {
    std::fs::read_to_string("res/day7.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn fuel(from: isize, to: isize) -> isize {
    let dis = (to - from).abs();
    dis * (dis + 1) / 2
}
