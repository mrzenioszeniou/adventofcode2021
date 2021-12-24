use crate::pf::a_star;
use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Display, Write},
};

pub fn solve() -> (usize, usize) {
    let start = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (Cave::Color(Color::A), vec![Color::D, Color::C]),
            (Cave::Color(Color::B), vec![Color::D, Color::C]),
            (Cave::Color(Color::C), vec![Color::B, Color::A]),
            (Cave::Color(Color::D), vec![Color::A, Color::B]),
        ]),
    };

    (part1(start), 42)
}

fn part1(start: Situation) -> usize {
    let target = Situation {
        caves: BTreeMap::from([
            (Cave::LL, vec![]),
            (Cave::L, vec![]),
            (Cave::AB, vec![]),
            (Cave::BC, vec![]),
            (Cave::CD, vec![]),
            (Cave::R, vec![]),
            (Cave::RR, vec![]),
            (Cave::Color(Color::A), vec![Color::A, Color::A]),
            (Cave::Color(Color::B), vec![Color::B, Color::B]),
            (Cave::Color(Color::C), vec![Color::C, Color::C]),
            (Cave::Color(Color::D), vec![Color::D, Color::D]),
        ]),
    };

    let path = a_star(start, target, |s| s.nexts(), |s| s.heuristic()).unwrap();

    for each in path.0 {
        println!("{}\n", each);
    }

    path.1
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

    pub fn to_char(&self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
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
                if let Cave::Color(color) = cave {
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
            Cave::Color(c) => self.caves.get(cave).unwrap().iter().all(|a| a == c),
            c => self.caves.get(c).unwrap().len() < c.capacity(),
        }
    }
}

impl Debug for Situation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n#")?;

        f.write_char(
            self.caves
                .get(&Cave::LL)
                .unwrap()
                .get(0)
                .map(|c| c.to_char())
                .unwrap_or('.'),
        )?;
        f.write_char(
            self.caves
                .get(&Cave::L)
                .unwrap()
                .get(0)
                .map(|c| c.to_char())
                .unwrap_or('.'),
        )?;
        f.write_char('.')?;
        for cave in [Cave::AB, Cave::BC, Cave::CD] {
            f.write_char(
                self.caves
                    .get(&cave)
                    .unwrap()
                    .get(0)
                    .map(|c| c.to_char())
                    .unwrap_or('.'),
            )?;
            f.write_char('.')?;
        }

        f.write_char(
            self.caves
                .get(&Cave::R)
                .unwrap()
                .get(0)
                .map(|c| c.to_char())
                .unwrap_or('.'),
        )?;
        f.write_char(
            self.caves
                .get(&Cave::RR)
                .unwrap()
                .get(0)
                .map(|c| c.to_char())
                .unwrap_or('.'),
        )?;
        f.write_str("#\n###")?;

        for color in [Color::A, Color::B, Color::C, Color::D] {
            f.write_char(
                self.caves
                    .get(&Cave::Color(color))
                    .unwrap()
                    .get(1)
                    .map(|c| c.to_char())
                    .unwrap_or('.'),
            )?;
            f.write_char('#')?;
        }
        f.write_str("##\n  #")?;

        for color in [Color::A, Color::B, Color::C, Color::D] {
            f.write_char(
                self.caves
                    .get(&Cave::Color(color))
                    .unwrap()
                    .get(0)
                    .map(|c| c.to_char())
                    .unwrap_or('.'),
            )?;
            f.write_char('#')?;
        }
        f.write_str("  \n  #########")
    }
}

impl Display for Situation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
    Color(Color),
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
            Self::Color(_) => 2,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        match self {
            Self::LL => (1, 1),
            Self::L => (1, 2),
            Self::Color(Color::A) => (3, 3),
            Self::AB => (1, 4),
            Self::Color(Color::B) => (3, 5),
            Self::BC => (1, 6),
            Self::Color(Color::C) => (3, 7),
            Self::CD => (1, 8),
            Self::Color(Color::D) => (3, 9),
            Self::R => (1, 10),
            Self::RR => (1, 11),
        }
    }

    pub fn color(&self) -> Option<&Color> {
        match self {
            Self::Color(c) => Some(c),
            _ => None,
        }
    }

    pub fn can_move(&self, other: &Self) -> Option<Vec<Cave>> {
        if self.color().is_some() && other.color().is_none() {
            return other.can_move(self);
        }

        Some(match (self, other) {
            (Cave::LL, Cave::Color(Color::A)) => vec![Self::L],
            (Cave::LL, Cave::Color(Color::B)) => vec![Self::L, Self::AB],
            (Cave::LL, Cave::Color(Color::C)) => vec![Self::L, Self::AB, Self::BC],
            (Cave::LL, Cave::Color(Color::D)) => vec![Self::L, Self::AB, Self::BC, Self::CD],
            (Cave::L, Cave::Color(Color::A)) => vec![],
            (Cave::L, Cave::Color(Color::B)) => vec![Self::AB],
            (Cave::L, Cave::Color(Color::C)) => vec![Self::AB, Self::BC],
            (Cave::L, Cave::Color(Color::D)) => vec![Self::AB, Self::BC, Self::CD],
            (Cave::AB, Cave::Color(Color::A)) => vec![],
            (Cave::AB, Cave::Color(Color::B)) => vec![],
            (Cave::AB, Cave::Color(Color::C)) => vec![Self::BC],
            (Cave::AB, Cave::Color(Color::D)) => vec![Self::BC, Self::CD],
            (Cave::BC, Cave::Color(Color::A)) => vec![Self::AB],
            (Cave::BC, Cave::Color(Color::B)) => vec![],
            (Cave::BC, Cave::Color(Color::C)) => vec![],
            (Cave::BC, Cave::Color(Color::D)) => vec![Self::CD],
            (Cave::CD, Cave::Color(Color::A)) => vec![Self::AB, Self::BC],
            (Cave::CD, Cave::Color(Color::B)) => vec![Self::BC],
            (Cave::CD, Cave::Color(Color::C)) => vec![],
            (Cave::CD, Cave::Color(Color::D)) => vec![],

            (Cave::R, Cave::Color(Color::A)) => vec![Self::AB, Self::BC, Self::CD],
            (Cave::R, Cave::Color(Color::B)) => vec![Self::BC, Self::CD],
            (Cave::R, Cave::Color(Color::C)) => vec![Self::CD],
            (Cave::R, Cave::Color(Color::D)) => vec![],

            (Cave::RR, Cave::Color(Color::A)) => vec![Self::R, Self::AB, Self::BC, Self::CD],
            (Cave::RR, Cave::Color(Color::B)) => vec![Self::R, Self::BC, Self::CD],
            (Cave::RR, Cave::Color(Color::C)) => vec![Self::R, Self::CD],
            (Cave::RR, Cave::Color(Color::D)) => vec![Self::R],
            // (Cave::Color(Color::A), Cave::Color(Color::B)) => vec![Self::AB],
            // (Cave::Color(Color::A), Cave::Color(Color::C)) => vec![Self::AB, Self::BC],
            // (Cave::Color(Color::A), Cave::Color(Color::D)) => vec![Self::AB, Self::BC, Self::CD],
            // (Cave::Color(Color::B), Cave::Color(Color::A)) => vec![Self::AB],
            // (Cave::Color(Color::B), Cave::Color(Color::C)) => vec![Self::BC],
            // (Cave::Color(Color::B), Cave::Color(Color::D)) => vec![Self::BC, Self::CD],
            // (Cave::Color(Color::C), Cave::Color(Color::A)) => vec![Self::AB, Self::BC],
            // (Cave::Color(Color::C), Cave::Color(Color::B)) => vec![Self::BC],
            // (Cave::Color(Color::C), Cave::Color(Color::D)) => vec![Self::CD],
            // (Cave::Color(Color::D), Cave::Color(Color::A)) => vec![Self::AB, Self::BC, Self::CD],
            // (Cave::Color(Color::D), Cave::Color(Color::B)) => vec![Self::BC, Self::CD],
            // (Cave::Color(Color::D), Cave::Color(Color::C)) => vec![Self::CD],
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
                (Cave::Color(Color::A), vec![Color::B, Color::A]),
                (Cave::Color(Color::B), vec![Color::A, Color::B]),
                (Cave::Color(Color::C), vec![Color::D, Color::C]),
                (Cave::Color(Color::D), vec![Color::C, Color::D]),
            ]),
        };

        assert_eq!(start.nexts().len(), 28);
    }

    #[test]
    fn examples() {
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        let start = Situation {
            caves: BTreeMap::from([
                (Cave::LL, vec![]),
                (Cave::L, vec![]),
                (Cave::AB, vec![]),
                (Cave::BC, vec![]),
                (Cave::CD, vec![]),
                (Cave::R, vec![]),
                (Cave::RR, vec![]),
                (Cave::Color(Color::A), vec![Color::A, Color::B]),
                (Cave::Color(Color::B), vec![Color::D, Color::C]),
                (Cave::Color(Color::C), vec![Color::C, Color::B]),
                (Cave::Color(Color::D), vec![Color::A, Color::D]),
            ]),
        };

        assert_eq!(part1(start), 12521);
    }
}
