use std::collections::HashMap;

pub fn solve() -> (u32, u32) {
    let (cards, numbers) = parse();

    (part1(cards.clone(), &numbers), part2(cards, &numbers))
}

fn part1(mut cards: Vec<Card>, numbers: &[u32]) -> u32 {
    for number in numbers {
        for card in cards.iter_mut() {
            if card.mark_number(*number) {
                return card.unmarked_sum() * number;
            }
        }
    }

    panic!("No winners were found");
}

fn part2(mut cards: Vec<Card>, numbers: &[u32]) -> u32 {
    for number in numbers {
        for i in (0..cards.len()).rev() {
            if cards[i].mark_number(*number) {
                if cards.len() == 1 {
                    return cards[i].unmarked_sum() * number;
                } else {
                    cards.remove(i);
                }
            }
        }
    }

    panic!("No winners were found");
}

fn parse() -> (Vec<Card>, Vec<u32>) {
    let content = std::fs::read_to_string("res/day4.txt").unwrap();

    let mut lines = content.split('\n');

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cards = vec![];
    let mut card = vec![];

    for line in lines {
        if line.is_empty() {
            cards.push(Card::new(card));
            card = vec![];
        } else {
            card.push(
                line.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        }
    }

    (cards, numbers)
}

#[derive(Clone)]
struct Card {
    numbers: Vec<Vec<Number>>,
    index: HashMap<u32, (usize, usize)>,
}

impl Card {
    pub fn new(raw: Vec<Vec<u32>>) -> Self {
        let mut index = HashMap::new();
        let mut numbers = vec![];

        for (i, raw_line) in raw.into_iter().enumerate() {
            let mut line = vec![];
            for (j, raw_number) in raw_line.into_iter().enumerate() {
                index.insert(raw_number, (i, j));
                line.push(raw_number.into());
            }
            numbers.push(line);
        }

        Self { numbers, index }
    }

    pub fn mark_number(&mut self, number: u32) -> bool {
        if let Some((i, j)) = self.index.get(&number) {
            self.numbers[*i][*j].mark();

            if self.numbers[*i].iter().all(|n| n.is_marked())
                || self.numbers.iter().all(|line| line[*j].is_marked())
            {
                return true;
            }
        }

        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .map(|l| l.iter())
            .flatten()
            .filter(|n| !n.is_marked())
            .map(|n| n.num())
            .sum()
    }
}

#[derive(Clone)]
enum Number {
    Marked(u32),
    Unmarked(u32),
}

impl From<u32> for Number {
    fn from(from: u32) -> Self {
        Self::Unmarked(from)
    }
}

impl Number {
    pub fn is_marked(&self) -> bool {
        match self {
            Self::Marked(_) => true,
            Self::Unmarked(_) => false,
        }
    }

    pub fn mark(&mut self) {
        *self = match self {
            Self::Marked(_) => panic!("Attempted to mark an already marked number"),
            Self::Unmarked(n) => Self::Marked(*n),
        }
    }

    pub fn num(&self) -> u32 {
        *match self {
            Self::Marked(num) => num,
            Self::Unmarked(num) => num,
        }
    }
}
