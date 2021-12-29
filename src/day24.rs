// Extracted from puzzle input
const CONSTS: [(isize, isize); 14] = [
    (10, 2),  // w0
    (10, 4),  // w1
    (14, 8),  // w2
    (11, 7),  // w3 🍟 pushing
    (14, 12), // w4 🍏 pushing
    (-14, 7), // w5 🍏 popping w4 + 12 - 14 == w5 => w4 == w5 + 2
    (0, 10),  // w6 🍟 popping w3 + 7 + 0 == w6 => w3 == w6 - 7
    (10, 14), // w7
    (-10, 2), // w8
    (13, 6),  // w9
    (-12, 8), // w10
    (-3, 11), // w11
    (-11, 5), // w12
    (-2, 11), // w13
];

pub fn solve() -> (isize, isize) {
    (
        solver(false)
            .into_iter()
            .enumerate()
            .map(|(i, d)| d * 10_isize.pow(i as u32))
            .sum(),
        solver(true)
            .into_iter()
            .enumerate()
            .map(|(i, d)| d * 10_isize.pow(i as u32))
            .sum(),
    )
}

fn solver(smallest: bool) -> Vec<isize> {
    let mut stack = vec![];
    let mut input = vec![0; 14];

    'outer: for (j, (left, _)) in CONSTS.into_iter().enumerate() {
        if left >= 1 {
            stack.push(j);
        } else {
            let i = stack.pop().unwrap();
            let pushed = CONSTS[i];
            let popped = CONSTS[j];

            let diff = pushed.1 + popped.0;

            let mut range: Vec<isize> = (1..=9).collect();

            if !smallest {
                range.reverse();
            }

            for n in range {
                let counterpart = n - diff;

                if let 1..=9 = counterpart {
                    input[i] = counterpart;
                    input[j] = n;
                    continue 'outer;
                }
            }

            panic!(
                "Did not find two valid numbers with a difference of {}",
                diff
            );
        }
    }

    input.reverse();

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
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

    #[test]
    fn examples() {
        let mut alu = ALU::default();
        alu.execute(
            &std::fs::read_to_string("res/day24.txt").unwrap(),
            solver(false),
        );
        assert_eq!(alu.z, 0);

        let mut alu = ALU::default();
        alu.execute(
            &std::fs::read_to_string("res/day24.txt").unwrap(),
            solver(true),
        );
        assert_eq!(alu.z, 0);
    }
}
