use std::ops::Not;

pub fn solve() -> (usize, usize) {
    let report = parse();

    (part1(&report), part2(&report))
}

fn part1(report: &[Vec<usize>]) -> usize {
    let n_bits = report[0].len();
    let mut gamma_rate = 0;

    for j in 0..n_bits {
        let count = report.iter().filter(|line| line[j] == 1).count();

        if count > report.len() / 2 {
            gamma_rate += 2_usize.pow((n_bits - 1 - j) as u32);
        }
    }

    let epsilon_rate = gamma_rate.not() % 2_usize.pow(n_bits as u32);

    gamma_rate * epsilon_rate
}

fn part2(report: &[Vec<usize>]) -> usize {
    let n_bits = report[0].len();
    let mut oxygen_options = report.to_owned();
    let mut co2_options = report.to_owned();

    for j in 0..n_bits {
        if oxygen_options.len() > 1 {
            let oxygen_ones = oxygen_options.iter().filter(|line| line[j] == 1).count();
            let oxygen_zeros = oxygen_options.len() - oxygen_ones;
            if oxygen_ones < oxygen_zeros {
                oxygen_options.retain(|line| line[j] == 0);
            } else {
                oxygen_options.retain(|line| line[j] == 1);
            }
        }

        if co2_options.len() > 1 {
            let co2_ones = co2_options.iter().filter(|line| line[j] == 1).count();
            let co2_zeros = co2_options.len() - co2_ones;
            if co2_ones < co2_zeros {
                co2_options.retain(|line| line[j] == 1);
            } else {
                co2_options.retain(|line| line[j] == 0);
            }
        }
    }

    assert_eq!(oxygen_options.len(), 1);
    assert_eq!(co2_options.len(), 1);

    let oxygen_rating: usize = oxygen_options[0]
        .iter()
        .enumerate()
        .map(|(j, digit)| digit * 2_usize.pow((n_bits - j - 1) as u32))
        .sum();

    let co2_rating: usize = co2_options[0]
        .iter()
        .enumerate()
        .map(|(j, digit)| digit * 2_usize.pow((n_bits - j - 1) as u32))
        .sum();

    oxygen_rating * co2_rating
}

fn parse() -> Vec<Vec<usize>> {
    std::fs::read_to_string("res/day3.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let report = [
            vec![0, 0, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1],
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];

        assert_eq!(part1(&report), 198);
        assert_eq!(part2(&report), 230);
    }
}
