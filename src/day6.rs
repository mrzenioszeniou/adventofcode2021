use std::collections::HashMap;

pub fn solve() -> (usize, usize) {
    let school = parse();

    (simulator(school.clone(), 80), simulator(school, 256))
}

fn simulator(mut school: HashMap<u8, usize>, days: usize) -> usize {
    for _ in 0..days {
        let mut next = HashMap::new();

        for (age, count) in school.into_iter() {
            if age == 0 {
                next.entry(6)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
                next.entry(8)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
            } else {
                next.entry(age - 1)
                    .and_modify(|cnt| *cnt += count)
                    .or_insert(count);
            }
        }

        school = next;
    }

    school.values().sum()
}

fn parse() -> HashMap<u8, usize> {
    std::fs::read_to_string("res/day6.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut map, fish| {
            map.entry(fish).and_modify(|cnt| *cnt += 1).or_insert(1);
            map
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example0() {
        assert_eq!(simulator(HashMap::from([(3, 1)]), 5), 2);
    }

    #[test]
    fn example1() {
        assert_eq!(
            simulator(HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]), 18),
            26
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            simulator(HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]), 80),
            5934
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            simulator(HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]), 256),
            26984457539
        );
    }
}
