pub fn solve() -> (isize, usize) {
    let program = std::fs::read_to_string("res/day24.txt").unwrap();

    (part1(&program), 42)
}

fn part1(program: &str) -> isize {
    let alu = ALU::new(program);

    let mut input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
    loop {
        println!("{:?}", input);

        let mut alu = alu.clone();

        alu.execute(input.clone());

        if alu.done() && alu.z == 0 {
            return input
                .into_iter()
                .enumerate()
                .map(|(i, d)| d * 10_isize.pow(i as u32))
                .sum();
        } else {
            for digit in input.iter_mut() {
                *digit += 1;

                if *digit > 9 {
                    *digit = 1;
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Clone)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
    index: usize,
    program: Vec<String>,
    input: Vec<isize>,
}

impl ALU {
    pub fn new(from: &str) -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            index: 0,
            program: from
                .split('\n')
                .map(|s| s.trim().to_string())
                .filter(|c| !c.is_empty())
                .collect(),
            input: vec![],
        }
    }

    fn done(&self) -> bool {
        self.index == self.program.len()
    }

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

    pub fn execute(&mut self, mut input: Vec<isize>) {
        while self.index < self.program.len() {
            if let Some(r) = sscanf::scanf!(self.program[self.index], "inp {}", String) {
                if let Some(input) = input.pop() {
                    self.input.push(input);
                    *self.register(&r) = input;
                } else {
                    return;
                }
            } else {
                let (command, left, right) =
                    sscanf::scanf!(self.program[self.index], "{} {} {}", String, String, String)
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

            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn examples() {}
}
