use std::{
    collections::{BTreeMap, HashMap, HashSet},
    slice::SliceIndex,
};

pub fn solve() -> (usize, usize) {
    (42, 42)
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    A,
    B,
    C,
    D,
}

impl Color {
    pub fn home_j(&self) -> usize {
        match self {
            Self::A => 3,
            Self::B => 5,
            Self::C => 7,
            Self::D => 9,
        }
    }

    pub fn energy(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::A, Self::B, Self::C, Self::D]
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Situation {
    ab: Option<Color>,
    bc: Option<Color>,
    cd: Option<Color>,
    left: Vec<Color>,
    right: Vec<Color>,
    caves: BTreeMap<Color, Vec<Color>>,
}

impl Situation {
    pub fn heuristic(&self) -> usize {
        Color::all()
            .iter()
            .flat_map(|c| {
                self.caves
                    .get(c)
                    .unwrap()
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(pos, a)| if *c != *a { a.energy() * (pos + 1) } else { 0 })
            })
            .sum()
    }

    pub fn nexts(&self) -> HashSet<(Self, usize)> {
        let mut ret = HashSet::new();

        // Pop the left cave
        let mut s = self.clone();
        if let Some(a) = s.left.pop() {
            if self.is_clean(&a) {
                let cave = s.caves.get_mut(&a).unwrap();

                let from_i = 1;
                let from_j = 1 + s.left.len();
                let to_i = 3 - cave.len();
                let to_j = a.home_j();

                let energy = (to_i.abs_diff(from_i) + to_j.abs_diff(from_j)) * a.energy();

                cave.push(a);
                ret.insert((s, energy));
            }
        }

        ret
    }

    fn is_clean(&self, color: &Color) -> bool {
        self.caves.get(color).unwrap().iter().all(|a| a == color)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn situation() {
        assert_eq!(
            Situation {
                caves: BTreeMap::from([
                    (Color::A, vec![Color::A, Color::A]),
                    (Color::B, vec![Color::B, Color::B]),
                    (Color::C, vec![Color::C, Color::C]),
                    (Color::D, vec![Color::D, Color::D]),
                ]),
                ab: None,
                bc: None,
                cd: None,
                left: vec![],
                right: vec![],
            }
            .heuristic(),
            0
        );

        assert_eq!(
            Situation {
                caves: BTreeMap::from([
                    (Color::A, vec![Color::A, Color::A]),
                    (Color::B, vec![Color::D, Color::B]),
                    (Color::C, vec![Color::C, Color::C]),
                    (Color::D, vec![Color::B, Color::D]),
                ]),
                ab: None,
                bc: None,
                cd: None,
                left: vec![],
                right: vec![],
            }
            .heuristic(),
            Color::D.energy() * 2 + Color::B.energy() * 2,
        );
    }
}
