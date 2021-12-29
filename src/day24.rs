const CONSTS: [(isize, isize); 14] = [
    (10, 2),  // w0
    (10, 4),  // w1
    (14, 8),  // w2
    (11, 7),  // w3 🎉 pushing
    (14, 12), // w4 ✅ pushing 9 + 12  - 14 == 7
    (-14, 7), // w5 ✅ popping w4 + 12 - 14 == w5 => w4 == w5 + 2
    (0, 10),  // w6 🎉 popping w3 + 7 + 0 == w6 => w3 == w6 - 7
    (10, 14), // w7
    (-10, 2), // w8
    (13, 6),  // w9
    (-12, 8), // w10
    (-3, 11), // w11
    (-11, 5), // w12
    (-2, 11), // w13
];

pub fn solve() -> (isize, usize) {
    // let program = std::fs::read_to_string("res/day24.txt").unwrap();

    (
        part1()
            .into_iter()
            .enumerate()
            .map(|(i, d)| d * 10_isize.pow(i as u32))
            .sum(),
        42,
    )
}

// fn part1(program: &str) -> isize {
//     // let mut input = vec![1; 14];

//     let mut min = isize::MAX;
//     let mut sol = None;

//     let options: Vec<Vec<isize>> = vec![
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (8..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//         (1..=9).rev().collect(),
//     ];

//     let mut indices = vec![0_usize; 14];

//     'outer: loop {
//         let input: Vec<isize> = indices
//             .iter()
//             .enumerate()
//             .map(|(d, i)| options[d][*i])
//             .collect();

//         // let mut alu = ALU::default();
//         // alu.execute(program, input.clone());
//         // let output = alu.z;
//         let output = execute(&input);

//         if output == 0 {
//             sol = Some(
//                 input
//                     .iter()
//                     .enumerate()
//                     .map(|(i, d)| d * 10_isize.pow(i as u32))
//                     .sum(),
//             );
//             min = 0;
//             break 'outer;
//         }
//         // if min >= output {
//         //     min = output;
//         //     sol = Some(
//         //         input
//         //             .iter()
//         //             .enumerate()
//         //             .map(|(i, d)| d * 10_isize.pow(i as u32))
//         //             .sum(),
//         //     );
//         //     println!("{:?} => {}", input, min);
//         //     if min == 0 {
//         //         break;
//         //     }
//         // }

//         for (digit, index) in indices.iter_mut().enumerate() {
//             *index += 1;

//             if *index >= options[digit].len() {
//                 *index = 0;

//                 if digit == input.len() - 1 {
//                     break 'outer;
//                 }
//             } else {
//                 break;
//             }
//         }
//     }

//     if min == 0 {
//         sol.unwrap()
//     } else {
//         panic!("No solution found");
//     }
// }

fn part1() -> Vec<isize> {
    let mut stack = vec![];
    let mut pairs = vec![];
    for (i, (left, _)) in CONSTS.into_iter().enumerate() {
        if left >= 1 {
            stack.push(i);
        } else {
            pairs.push((stack.pop().unwrap(), i));
        }
    }

    let mut input = vec![0; 14];
    'pair: for pair in pairs {
        println!(
            "⬇ {} {:?} ⬆ {} {:?}",
            pair.0, CONSTS[pair.0], pair.1, CONSTS[pair.1],
        );
        let diff = CONSTS[pair.0].1 + CONSTS[pair.1].0;

        for n in (1..=9).rev() {
            let counterpart = n - diff;

            if let 1..=9 = counterpart {
                input[pair.0] = counterpart;
                input[pair.1] = n;
                continue 'pair;
            }
        }

        panic!(
            "Did not find two valid numbers with a difference of {}",
            diff
        );
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
        alu.execute(&std::fs::read_to_string("res/day24.txt").unwrap(), part1());
        assert_eq!(alu.z, 0);
    }
}
