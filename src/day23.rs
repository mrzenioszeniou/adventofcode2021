use crate::pf::a_star;
use std::collections::{BTreeMap, HashSet};

pub fn solve() -> (usize, usize) {
    let start_1 = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (Cave::Color(Color::A, 2), vec![Color::D, Color::C]),
            (Cave::Color(Color::B, 2), vec![Color::D, Color::C]),
            (Cave::Color(Color::C, 2), vec![Color::B, Color::A]),
            (Cave::Color(Color::D, 2), vec![Color::A, Color::B]),
        ]),
    };

    let target_1 = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (Cave::Color(Color::A, 2), vec![Color::A, Color::A]),
            (Cave::Color(Color::B, 2), vec![Color::B, Color::B]),
            (Cave::Color(Color::C, 2), vec![Color::C, Color::C]),
            (Cave::Color(Color::D, 2), vec![Color::D, Color::D]),
        ]),
    };

    let start_2 = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (
                Cave::Color(Color::A, 4),
                vec![Color::D, Color::D, Color::D, Color::C],
            ),
            (
                Cave::Color(Color::B, 4),
                vec![Color::D, Color::B, Color::C, Color::C],
            ),
            (
                Cave::Color(Color::C, 4),
                vec![Color::B, Color::A, Color::B, Color::A],
            ),
            (
                Cave::Color(Color::D, 4),
                vec![Color::A, Color::C, Color::A, Color::B],
            ),
        ]),
    };

    let target_2 = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (Cave::Color(Color::A, 4), vec![Color::A; 4]),
            (Cave::Color(Color::B, 4), vec![Color::B; 4]),
            (Cave::Color(Color::C, 4), vec![Color::C; 4]),
            (Cave::Color(Color::D, 4), vec![Color::D; 4]),
        ]),
    };

    (solver(start_1, target_1), solver(start_2, target_2))
}

fn solver(start: Situation, target: Situation) -> usize {
    a_star(start, target, |s| s.nexts(), |s| s.heuristic())
        .unwrap()
        .1
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    A,
    B,
    C,
    D,
}

impl Color {
    pub fn energy(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Situation {
    caves: BTreeMap<Cave, Vec<Color>>,
}

impl Situation {
    pub fn heuristic(&self) -> usize {
        self.caves
            .iter()
            .map(|(cave, ampipods)| {
                if let Cave::Color(color, _) = cave {
                    ampipods
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(pos, a)| {
                            if *color != *a {
                                a.energy() * (pos + 1)
                            } else {
                                0
                            }
                        })
                        .sum()
                } else {
                    ampipods
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(pos, a)| a.energy() * (pos + 1))
                        .sum::<usize>()
                }
            })
            .sum()
    }

    pub fn nexts(&self) -> HashSet<(Self, usize)> {
        let mut ret = HashSet::new();

        for (from, from_pods) in self.caves.iter() {
            for (to, to_pods) in self.caves.iter() {
                if from != to
                    && !from_pods.is_empty()
                    && self.is_clean(to)
                    && from
                        .can_move(to)
                        .map(|i| i.iter().all(|c| self.is_clean(c)))
                        .unwrap_or(false)
                {
                    let from_pos = from.pos();
                    let to_pos = to.pos();

                    let dist = to_pos.0.abs_diff(from_pos.0) + to_pos.1.abs_diff(from_pos.1)
                        - to_pods.len()
                        - (from_pods.len() - 1);

                    let mut next = self.clone();

                    let amphipod = next.caves.get_mut(from).unwrap().pop().unwrap();

                    if to.color().map(|c| c != &amphipod).unwrap_or(false) {
                        continue;
                    }

                    let energy = dist * amphipod.energy();

                    next.caves.get_mut(to).unwrap().push(amphipod);

                    ret.insert((next, energy));
                }
            }
        }

        ret
    }

    pub fn is_clean(&self, cave: &Cave) -> bool {
        match cave {
            Cave::Color(c, _) => self.caves.get(cave).unwrap().iter().all(|a| a == c),
            c => self.caves.get(c).unwrap().len() < c.capacity(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Cave {
    L,
    LL,
    AB,
    BC,
    CD,
    R,
    RR,
    Color(Color, usize),
}

impl Cave {
    pub fn capacity(&self) -> usize {
        match self {
            Self::LL => 1,
            Self::L => 1,
            Self::AB => 1,
            Self::BC => 1,
            Self::CD => 1,
            Self::R => 1,
            Self::RR => 1,
            Self::Color(_, cap) => *cap,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        match self {
            Self::LL => (1, 1),
            Self::L => (1, 2),
            Self::Color(Color::A, cap) => (1 + cap, 3),
            Self::AB => (1, 4),
            Self::Color(Color::B, cap) => (1 + cap, 5),
            Self::BC => (1, 6),
            Self::Color(Color::C, cap) => (1 + cap, 7),
            Self::CD => (1, 8),
            Self::Color(Color::D, cap) => (1 + cap, 9),
            Self::R => (1, 10),
            Self::RR => (1, 11),
        }
    }

    pub fn color(&self) -> Option<&Color> {
        match self {
            Self::Color(c, _) => Some(c),
            _ => None,
        }
    }

    pub fn can_move(&self, other: &Self) -> Option<Vec<Cave>> {
        if self.color().is_some() && other.color().is_none() {
            return other.can_move(self);
        }

        Some(match (self, other) {
            (Cave::LL, Cave::Color(Color::A, _)) => vec![Self::L],
            (Cave::LL, Cave::Color(Color::B, _)) => vec![Self::L, Self::AB],
            (Cave::LL, Cave::Color(Color::C, _)) => vec![Self::L, Self::AB, Self::BC],
            (Cave::LL, Cave::Color(Color::D, _)) => vec![Self::L, Self::AB, Self::BC, Self::CD],
            (Cave::L, Cave::Color(Color::A, _)) => vec![],
            (Cave::L, Cave::Color(Color::B, _)) => vec![Self::AB],
            (Cave::L, Cave::Color(Color::C, _)) => vec![Self::AB, Self::BC],
            (Cave::L, Cave::Color(Color::D, _)) => vec![Self::AB, Self::BC, Self::CD],
            (Cave::AB, Cave::Color(Color::A, _)) => vec![],
            (Cave::AB, Cave::Color(Color::B, _)) => vec![],
            (Cave::AB, Cave::Color(Color::C, _)) => vec![Self::BC],
            (Cave::AB, Cave::Color(Color::D, _)) => vec![Self::BC, Self::CD],
            (Cave::BC, Cave::Color(Color::A, _)) => vec![Self::AB],
            (Cave::BC, Cave::Color(Color::B, _)) => vec![],
            (Cave::BC, Cave::Color(Color::C, _)) => vec![],
            (Cave::BC, Cave::Color(Color::D, _)) => vec![Self::CD],
            (Cave::CD, Cave::Color(Color::A, _)) => vec![Self::AB, Self::BC],
            (Cave::CD, Cave::Color(Color::B, _)) => vec![Self::BC],
            (Cave::CD, Cave::Color(Color::C, _)) => vec![],
            (Cave::CD, Cave::Color(Color::D, _)) => vec![],
            (Cave::R, Cave::Color(Color::A, _)) => vec![Self::AB, Self::BC, Self::CD],
            (Cave::R, Cave::Color(Color::B, _)) => vec![Self::BC, Self::CD],
            (Cave::R, Cave::Color(Color::C, _)) => vec![Self::CD],
            (Cave::R, Cave::Color(Color::D, _)) => vec![],
            (Cave::RR, Cave::Color(Color::A, _)) => vec![Self::R, Self::AB, Self::BC, Self::CD],
            (Cave::RR, Cave::Color(Color::B, _)) => vec![Self::R, Self::BC, Self::CD],
            (Cave::RR, Cave::Color(Color::C, _)) => vec![Self::R, Self::CD],
            (Cave::RR, Cave::Color(Color::D, _)) => vec![Self::R],
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn nexts() {
        let start = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (Cave::Color(Color::A, 2), vec![Color::B, Color::A]),
                (Cave::Color(Color::B, 2), vec![Color::A, Color::B]),
                (Cave::Color(Color::C, 2), vec![Color::D, Color::C]),
                (Cave::Color(Color::D, 2), vec![Color::C, Color::D]),
            ]),
        };

        assert_eq!(start.nexts().len(), 28);
    }

    #[test]
    fn examples() {
        let start = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (Cave::Color(Color::A, 2), vec![Color::A, Color::B]),
                (Cave::Color(Color::B, 2), vec![Color::D, Color::C]),
                (Cave::Color(Color::C, 2), vec![Color::C, Color::B]),
                (Cave::Color(Color::D, 2), vec![Color::A, Color::D]),
            ]),
        };

        let target = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (Cave::Color(Color::A, 2), vec![Color::A; 2]),
                (Cave::Color(Color::B, 2), vec![Color::B; 2]),
                (Cave::Color(Color::C, 2), vec![Color::C; 2]),
                (Cave::Color(Color::D, 2), vec![Color::D; 2]),
            ]),
        };

        assert_eq!(solver(start, target), 12521);

        let start = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (
                    Cave::Color(Color::A, 4),
                    vec![Color::A, Color::D, Color::D, Color::B],
                ),
                (
                    Cave::Color(Color::B, 4),
                    vec![Color::D, Color::B, Color::C, Color::C],
                ),
                (
                    Cave::Color(Color::C, 4),
                    vec![Color::C, Color::A, Color::B, Color::B],
                ),
                (
                    Cave::Color(Color::D, 4),
                    vec![Color::A, Color::C, Color::A, Color::D],
                ),
            ]),
        };

        let target = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (Cave::Color(Color::A, 4), vec![Color::A; 4]),
                (Cave::Color(Color::B, 4), vec![Color::B; 4]),
                (Cave::Color(Color::C, 4), vec![Color::C; 4]),
                (Cave::Color(Color::D, 4), vec![Color::D; 4]),
            ]),
        };

        assert_eq!(solver(start, target), 44169);
    }
}
