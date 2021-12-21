use std::collections::HashMap;

pub fn solve() -> (usize, usize) {
    (part1(10, 4), part2(10, 4))
}

fn part1(p1: usize, p2: usize) -> usize {
    let mut players = vec![Player::new(p1), Player::new(p2)];
    let mut dice = Deterministic::default();
    let mut dice_rolls = 0;

    while !players.iter().any(|p| p.won()) {
        players[0].play(&mut dice);
        players.rotate_left(1);
        dice_rolls += 3;
    }

    players.iter().find(|p| !p.won()).unwrap().score * dice_rolls
}

fn part2(p1: usize, p2: usize) -> usize {
    let possibilitites = possibilities();

    let result = play(Player::new(p1), Player::new(p2), true, &possibilitites);

    std::cmp::max(result.0, result.1)
}

fn play(
    p1: Player,
    p2: Player,
    p1_next: bool,
    possibilitites: &HashMap<usize, usize>,
) -> (usize, usize) {
    if p1.score >= 21 {
        return (1, 0);
    }

    if p2.score >= 21 {
        return (0, 1);
    }

    let mut ret = (0, 0);

    for (step, count) in possibilitites.iter() {
        let mut p1 = p1.clone();
        let mut p2 = p2.clone();

        if p1_next {
            p1.forward(*step);
        } else {
            p2.forward(*step);
        }

        let result = play(p1, p2, !p1_next, possibilitites);

        ret.0 += result.0 * count;
        ret.1 += result.1 * count;
    }

    ret
}

fn possibilities() -> HashMap<usize, usize> {
    let mut ret = HashMap::new();

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                *ret.entry(i + j + k).or_default() += 1;
            }
        }
    }

    ret
}

#[derive(Clone)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    pub fn new(pos: usize) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }

    pub fn play<D: Dice>(&mut self, dice: &mut D) {
        self.forward(dice.roll() + dice.roll() + dice.roll());
    }

    pub fn forward(&mut self, steps: usize) {
        self.pos = (self.pos + steps) % 10;
        self.score += self.pos + 1;
    }

    pub fn won(&self) -> bool {
        self.score >= 1000
    }
}

trait Dice {
    fn roll(&mut self) -> usize;
}

#[derive(Default)]
struct Deterministic {
    curr: usize,
}

impl Dice for Deterministic {
    fn roll(&mut self) -> usize {
        let ret = self.curr;
        self.curr = (self.curr + 1) % 100;
        ret + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        println!("{:?}", possibilities());

        assert_eq!(part1(4, 8), 739785);
        assert_eq!(part2(4, 8), 444356092776315);
    }
}
