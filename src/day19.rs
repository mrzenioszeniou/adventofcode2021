use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

pub fn solve() -> (usize, isize) {
    let scanners = parse(std::fs::read_to_string("res/day19.txt").unwrap());

    (part1(&scanners), part2(&scanners))
}

fn part1(scanners: &[Vec<Vector>]) -> usize {
    let mut beacons: HashSet<Vector> = HashSet::from_iter(scanners[0].iter().cloned());

    for (scanner, (matrix, vector)) in match_scanners(scanners) {
        beacons.extend(
            scanners[scanner]
                .iter()
                .map(|beacon| *beacon * matrix + vector),
        );
    }

    beacons.len()
}

fn part2(scanners: &[Vec<Vector>]) -> isize {
    let transformations = match_scanners(scanners);

    let mut max = 0;

    for (_, (_, left)) in transformations.iter() {
        for (_, (_, right)) in transformations.iter() {
            let d = (left.0 - right.0).abs() + (left.1 - right.1).abs() + (left.2 - right.2).abs();
            max = std::cmp::max(max, d);
        }
    }

    max
}

/// Checks two sets of beacons for overlaps
fn compare(left_beacons: &[Vector], right_beacons: &[Vector]) -> Option<(Matrix, Vector)> {
    for rotation in Matrix::rotations() {
        let mut shifts: HashMap<Vector, usize> = HashMap::new();

        for left_beacon in left_beacons {
            for right_beacon in right_beacons {
                let transformed = *right_beacon * rotation;
                *shifts.entry(*left_beacon - transformed).or_default() += 1;
            }
        }

        if let Some((shift, _)) = shifts.into_iter().find(|(_, cnt)| *cnt >= 12) {
            return Some((rotation, shift));
        }
    }

    None
}

/// Attempts to match scanners in pairs based on their scanned beacons. Returns a rotation and shift
/// for each one, in relation to scanner 0
fn match_scanners(scanners: &[Vec<Vector>]) -> HashMap<usize, (Matrix, Vector)> {
    let mut matches: BTreeMap<usize, HashSet<(usize, Matrix, Vector)>> = BTreeMap::new();

    for (scanner, beacons) in scanners.iter().enumerate() {
        for (ref_scanner, ref_beacons) in scanners.iter().enumerate() {
            if scanner == ref_scanner {
                continue;
            }

            if let Some((rotation, shift)) = compare(ref_beacons, beacons) {
                assert!(matches
                    .entry(scanner)
                    .or_default()
                    .insert((ref_scanner, rotation, shift)));
            }
        }
    }

    (1..scanners.len())
        .map(|s| {
            (
                s,
                transformation(s, 0, HashSet::default(), &matches)
                    .unwrap_or_else(|| panic!("No match found for scanner {}", s)),
            )
        })
        .collect()

    // let transformations = HashMap::new();

    // for scanner in 1..scanners.len() {
    //     let (matrix, vector) = transformation(scanner, 0, HashSet::new(), &matches)
    //         .unwrap_or_else(|| panic!("No match found for scanner {}", scanner));

    //     beacons.extend(
    //         scanners[scanner]
    //             .iter()
    //             .map(|beacon| *beacon * matrix + vector),
    //     );
    // }

    // matches
}

/// Derives a transformation (Rotation + Shift) based on the given associations between 2 scanners
fn transformation(
    curr: usize,
    target: usize,
    history: HashSet<usize>,
    matches: &BTreeMap<usize, HashSet<(usize, Matrix, Vector)>>,
) -> Option<(Matrix, Vector)> {
    for (to, matrix, vector) in matches.get(&curr)?.iter() {
        if *to == target {
            return Some((*matrix, *vector));
        } else if history.contains(to) {
            continue;
        } else {
            let mut history = history.clone();
            history.insert(curr);
            if let Some((submatrix, subvector)) = transformation(*to, target, history, matches) {
                return Some((submatrix * *matrix, *vector * submatrix + subvector));
            }
        }
    }

    None
}

fn parse(from: String) -> Vec<Vec<Vector>> {
    let mut scanners = vec![];
    let mut scanner = vec![];

    for line in from.split('\n') {
        if line.trim().is_empty() {
            scanners.push(scanner);
            scanner = vec![];
        } else {
            scanner.push(Vector::parse(line).unwrap());
        }
    }

    scanners.push(scanner);

    scanners
}

type Dimension = isize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector(Dimension, Dimension, Dimension);
impl Vector {
    pub fn parse(from: &str) -> Option<Self> {
        let mut split = from.split(',');

        let x = split.next()?.parse().ok()?;
        let y = split.next()?.parse().ok()?;
        let z = split.next()?.parse().ok()?;

        Some(Self(x, y, z))
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Matrix(Vector, Vector, Vector);

impl Matrix {
    pub const fn rotations() -> [Self; 24] {
        [
            Matrix(Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1)),
            Matrix(Vector(1, 0, 0), Vector(0, 0, -1), Vector(0, 1, 0)),
            Matrix(Vector(1, 0, 0), Vector(0, -1, 0), Vector(0, 0, -1)),
            Matrix(Vector(1, 0, 0), Vector(0, 0, 1), Vector(0, -1, 0)),
            /////////////////
            Matrix(Vector(0, 1, 0), Vector(-1, 0, 0), Vector(0, 0, 1)),
            Matrix(Vector(0, 0, -1), Vector(-1, 0, 0), Vector(0, 1, 0)),
            Matrix(Vector(0, -1, 0), Vector(-1, 0, 0), Vector(0, 0, -1)),
            Matrix(Vector(0, 0, 1), Vector(-1, 0, 0), Vector(0, -1, 0)),
            ////////////////
            Matrix(Vector(-1, 0, 0), Vector(0, -1, 0), Vector(0, 0, 1)),
            Matrix(Vector(-1, 0, 0), Vector(0, 0, 1), Vector(0, 1, 0)),
            Matrix(Vector(-1, 0, 0), Vector(0, 1, 0), Vector(0, 0, -1)),
            Matrix(Vector(-1, 0, 0), Vector(0, 0, -1), Vector(0, -1, 0)),
            ///////////////
            Matrix(Vector(0, -1, 0), Vector(1, 0, 0), Vector(0, 0, 1)),
            Matrix(Vector(0, 0, 1), Vector(1, 0, 0), Vector(0, 1, 0)),
            Matrix(Vector(0, 1, 0), Vector(1, 0, 0), Vector(0, 0, -1)),
            Matrix(Vector(0, 0, -1), Vector(1, 0, 0), Vector(0, -1, 0)),
            //////////////
            Matrix(Vector(0, 0, -1), Vector(0, 1, 0), Vector(1, 0, 0)),
            Matrix(Vector(0, -1, 0), Vector(0, 0, -1), Vector(1, 0, 0)),
            Matrix(Vector(0, 0, 1), Vector(0, -1, 0), Vector(1, 0, 0)),
            Matrix(Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 0, 0)),
            //////////////
            Matrix(Vector(0, 0, 1), Vector(0, 1, 0), Vector(-1, 0, 0)),
            Matrix(Vector(0, 1, 0), Vector(0, 0, -1), Vector(-1, 0, 0)),
            Matrix(Vector(0, 0, -1), Vector(0, -1, 0), Vector(-1, 0, 0)),
            Matrix(Vector(0, -1, 0), Vector(0, 0, 1), Vector(-1, 0, 0)),
        ]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let line0 = Vector(
            self.0 .0 * other.0 .0 + self.0 .1 * other.1 .0 + self.0 .2 * other.2 .0,
            self.0 .0 * other.0 .1 + self.0 .1 * other.1 .1 + self.0 .2 * other.2 .1,
            self.0 .0 * other.0 .2 + self.0 .1 * other.1 .2 + self.0 .2 * other.2 .2,
        );
        let line1 = Vector(
            self.1 .0 * other.0 .0 + self.1 .1 * other.1 .0 + self.1 .2 * other.2 .0,
            self.1 .0 * other.0 .1 + self.1 .1 * other.1 .1 + self.1 .2 * other.2 .1,
            self.1 .0 * other.0 .2 + self.1 .1 * other.1 .2 + self.1 .2 * other.2 .2,
        );
        let line2 = Vector(
            self.2 .0 * other.0 .0 + self.2 .1 * other.1 .0 + self.2 .2 * other.2 .0,
            self.2 .0 * other.0 .1 + self.2 .1 * other.1 .1 + self.2 .2 * other.2 .1,
            self.2 .0 * other.0 .2 + self.2 .1 * other.1 .2 + self.2 .2 * other.2 .2,
        );
        Self(line0, line1, line2)
    }
}

impl Mul<Matrix> for Vector {
    type Output = Self;

    fn mul(self, matrix: Matrix) -> Self::Output {
        let x = self.0 * matrix.0 .0 + self.1 * matrix.0 .1 + self.2 * matrix.0 .2;
        let y = self.0 * matrix.1 .0 + self.1 * matrix.1 .1 + self.2 * matrix.1 .2;
        let z = self.0 * matrix.2 .0 + self.1 * matrix.2 .1 + self.2 * matrix.2 .2;
        Vector(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let a = Matrix(Vector(4, -8, 9), Vector(-1, 1, 3), Vector(3, 2, -1));
        let b = Matrix(Vector(3, -9, 2), Vector(2, -4, 1), Vector(1, 1, -7));
        let c = Matrix(Vector(5, 5, -63), Vector(2, 8, -22), Vector(12, -36, 15));

        assert_eq!(a * b, c);
    }

    #[test]
    fn examples() {
        let scanners = parse(std::fs::read_to_string("res/day19-example.txt").unwrap());

        let (r1, s1) =
            compare(&scanners[0], &scanners[1]).expect("Scanners 0 and 1 should overlap");

        assert_eq!(s1, Vector(68, -1246, -43));

        let (_, s3) = compare(&scanners[1], &scanners[3]).expect("Scanners 1 and 3 should overlap");

        let p3 = s3 * r1 + s1;

        assert_eq!(p3, Vector(-92, -2380, -20));

        let (r4, s4) =
            compare(&scanners[1], &scanners[4]).expect("Scanners 1 and 4 should overlap");

        let p4 = s4 * r1 + s1;

        assert_eq!(p4, Vector(-20, -1133, 1061));

        let (_, s2) = compare(&scanners[4], &scanners[2]).expect("Scanners 2 and 4 should overlap");

        let p2 = s2 * r4 * r1 + p4;

        assert_eq!(p2, Vector(1105, -1205, 1229));

        assert_eq!(part1(&scanners), 79);
        assert_eq!(part2(&scanners), 3621);
    }
}
