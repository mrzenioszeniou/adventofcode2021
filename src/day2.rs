pub fn solve() -> (usize, usize) {
    let steps = parse();

    (part1(&steps), part2(&steps))
}

fn part1(steps: &[Move]) -> usize {
    let mut depth = 0;
    let mut position = 0;

    for step in steps {
        match step {
            Move::Up(n) => depth -= n,
            Move::Down(n) => depth += n,
            Move::Forward(n) => position += n,
        }
    }

    depth * position
}

fn part2(steps: &[Move]) -> usize {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for step in steps {
        match step {
            Move::Up(n) => aim -= n,
            Move::Down(n) => aim += n,
            Move::Forward(n) => {
                position += n;
                depth += n * aim;
            }
        }
    }

    depth * position
}

fn parse() -> Vec<Move> {
    std::fs::read_to_string("res/day2.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let mut split = s.split_ascii_whitespace();

            let dir = split.next().unwrap().trim().to_ascii_lowercase();
            let n = split
                .next()
                .unwrap()
                .trim()
                .to_ascii_lowercase()
                .parse()
                .unwrap();

            match dir.as_str() {
                "up" => Move::Up(n),
                "down" => Move::Down(n),
                "forward" => Move::Forward(n),
                _ => panic!("Unexpected direction '{}' provided", dir),
            }
        })
        .collect()
}

enum Move {
    Up(usize),
    Down(usize),
    Forward(usize),
}
