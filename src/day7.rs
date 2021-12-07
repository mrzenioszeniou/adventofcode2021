use std::cmp::Ordering;

pub fn solve() -> (isize, isize) {
    let crabs = parse();

    (
        solver(&crabs, |from, to| (to - from).abs()),
        solver(&crabs, |from, to| {
            let dis = (to - from).abs();
            dis * (dis + 1) / 2
        }),
    )
}

fn solver(crabs: &[isize], fuel_calc: impl Fn(isize, isize) -> isize) -> isize {
    let mut curr = crabs[crabs.len() / 2];
    let mut curr_fuel = crabs.iter().map(|c| fuel_calc(*c, curr)).sum();

    loop {
        let left_fuel: isize = crabs.iter().map(|c| fuel_calc(*c, curr - 1)).sum();
        let right_fuel: isize = crabs.iter().map(|c| fuel_calc(*c, curr + 1)).sum();

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
