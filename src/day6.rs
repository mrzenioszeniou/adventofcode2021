pub fn solve() -> (usize, usize) {
    let school = parse();

    (simulator(school.clone(), 80), simulator(school, 256))
}

fn simulator(mut school: Vec<usize>, days: usize) -> usize {
    for _ in 0..days {
        school[7] += school[0];
        school.rotate_left(1);
    }

    school.iter().sum()
}

fn parse() -> Vec<usize> {
    std::fs::read_to_string("res/day6.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .fold(vec![0; 9], |mut acc, fish| {
            acc[fish] += 1;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example0() {
        assert_eq!(simulator(vec![0, 0, 0, 1, 0, 0, 0, 0, 0], 5), 2);
    }

    #[test]
    fn example1() {
        assert_eq!(simulator(vec![0, 1, 1, 2, 1, 0, 0, 0, 0], 18), 26);
    }

    #[test]
    fn example2() {
        assert_eq!(simulator(vec![0, 1, 1, 2, 1, 0, 0, 0, 0], 80), 5934);
    }

    #[test]
    fn example3() {
        assert_eq!(simulator(vec![0, 1, 1, 2, 1, 0, 0, 0, 0], 256), 26984457539);
    }
}
