use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn forward(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    pub fn iter_all() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
    }
}

pub fn neighbours_bounded(pos: (usize, usize), n: usize, m: usize) -> HashSet<(usize, usize)> {
    let pos = (pos.0 as isize, pos.1 as isize);
    let n = n as isize;
    let m = m as isize;

    Direction::iter_all()
        .map(|dir| {
            let step = dir.forward();
            (pos.0 + step.0, pos.1 + step.1)
        })
        .filter(|&(i, j)| i >= 0 && i < n && j >= 0 && j < m)
        .map(|(i, j)| (i as usize, j as usize))
        .collect()
}

pub fn neighbours_bounded_diagonal(
    pos: (usize, usize),
    n: usize,
    m: usize,
) -> HashSet<(usize, usize)> {
    let pos = (pos.0 as isize, pos.1 as isize);
    let n = n as isize;
    let m = m as isize;

    [
        (Direction::North, Direction::West),
        (Direction::North, Direction::East),
        (Direction::South, Direction::West),
        (Direction::South, Direction::East),
    ]
    .into_iter()
    .map(|(a, b)| {
        let step_a = a.forward();
        let step_b = b.forward();

        (pos.0 + step_a.0 + step_b.0, pos.1 + step_a.1 + step_b.1)
    })
    .filter(|&(i, j)| i >= 0 && i < n && j >= 0 && j < m)
    .map(|(i, j)| (i as usize, j as usize))
    .collect()
}
