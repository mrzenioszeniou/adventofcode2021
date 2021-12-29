pub fn solve() -> (isize, usize) {
    let program = std::fs::read_to_string("res/day24.txt").unwrap();

    (part1(&program), 42)
}

fn part1(program: &str) -> isize {
    // let mut input = vec![1; 14];

    let mut min = isize::MAX;
    let mut sol = None;

    let options: Vec<Vec<isize>> = vec![
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (8..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
        (1..=9).rev().collect(),
    ];

    let mut indices = vec![0_usize; 14];

    'outer: loop {
        let input: Vec<isize> = indices
            .iter()
            .enumerate()
            .map(|(d, i)| options[d][*i])
            .collect();

        // let mut alu = ALU::default();
        // alu.execute(program, input.clone());
        // let output = alu.z;
        let output = execute(&input);

        if output == 0 {
            sol = Some(
                input
                    .iter()
                    .enumerate()
                    .map(|(i, d)| d * 10_isize.pow(i as u32))
                    .sum(),
            );
            min = 0;
            break 'outer;
        }
        // if min >= output {
        //     min = output;
        //     sol = Some(
        //         input
        //             .iter()
        //             .enumerate()
        //             .map(|(i, d)| d * 10_isize.pow(i as u32))
        //             .sum(),
        //     );
        //     println!("{:?} => {}", input, min);
        //     if min == 0 {
        //         break;
        //     }
        // }

        for (digit, index) in indices.iter_mut().enumerate() {
            *index += 1;

            if *index >= options[digit].len() {
                *index = 0;

                if digit == input.len() - 1 {
                    break 'outer;
                }
            } else {
                break;
            }
        }
    }

    if min == 0 {
        sol.unwrap()
    } else {
        panic!("No solution found");
    }
}

fn execute(input: &[isize]) -> isize {
    const CONSTS: [(isize, isize); 14] = [
        (10, 2),
        (10, 4),
        (14, 8),
        (11, 7),
        (14, 12),
        (-14, 7),
        (0, 10),
        (10, 14),
        (-10, 2),
        (13, 6),
        (-12, 8),
        (-3, 11),
        (-11, 5),
        (-2, 11),
    ];

    assert_eq!(input.len(), CONSTS.len());

    let mut z = 0;

    for ((left, right), w) in CONSTS.into_iter().zip(input.iter().rev()) {
        let x = z % 26 + left;

        if left < 1 {
            z /= 26;
        }

        if x != *w {
            z = z * 26 + w + right;
        }
    }

    z
}

#[derive(Clone, Default)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl ALU {
    fn register(&mut self, register: &str) -> &mut isize {
        match register {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            _ => panic!("Invalid register requested: '{}'", register),
        }
    }

    fn value(&self, value: &str) -> isize {
        match value {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            n => n.parse().unwrap(),
        }
    }

    pub fn execute(&mut self, program: &str, mut input: Vec<isize>) {
        for line in program.split('\n') {
            let command = line.trim();

            if let Some(r) = sscanf::scanf!(command, "inp {}", String) {
                *self.register(&r) = input.pop().unwrap();
            } else {
                let (command, left, right) =
                    sscanf::scanf!(command, "{} {} {}", String, String, String)
                        .expect("Invalid command");

                match command.as_str() {
                    "mul" => *self.register(&left) = self.value(&left) * self.value(&right),
                    "add" => *self.register(&left) = self.value(&left) + self.value(&right),
                    "div" => *self.register(&left) = self.value(&left) / self.value(&right),
                    "mod" => *self.register(&left) = self.value(&left) % self.value(&right),
                    "eql" => {
                        *self.register(&left) = if self.value(&left) == self.value(&right) {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(execute(&[5, 2, 8, 3, 9, 6, 2, 1, 1, 1, 1, 1, 1, 1]), 2167);

        assert_eq!(execute(&[9, 9, 9, 1, 1, 5, 1, 1, 1, 1, 1, 1, 1, 1]), 56350);

        assert_eq!(
            execute(&[1, 3, 3, 2, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            1465111
        );
    }
}
