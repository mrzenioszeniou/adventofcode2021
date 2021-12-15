use std::collections::{BTreeSet, HashMap, HashSet};

use crate::dir::neighbours_bounded;

pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a.abs();
    }

    gcd(b, a % b)
}

/// The elements in each HashSet correspond to possible values we want to associate each key with.
/// `unique_values` if true, each value will only be present once in each combination
pub fn product<K, V>(
    mut possibilities: HashMap<K, HashSet<V>>,
    unique_values: bool,
) -> Vec<HashMap<K, V>>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    if possibilities.is_empty() {
        return vec![HashMap::new()];
    }

    let next = possibilities.keys().next().unwrap().clone();

    let mut ret = vec![];

    for possibility in possibilities.remove(&next).unwrap() {
        let mut cleaned_possibilities = possibilities.clone();

        cleaned_possibilities.values_mut().for_each(|p| {
            p.remove(&possibility);
        });

        let mut combos = product(cleaned_possibilities, unique_values);

        combos.iter_mut().for_each(|combo| {
            combo.insert(next.clone(), possibility.clone());
        });

        ret.append(&mut combos);
    }

    ret
}

/// A* search implementation.
pub fn a_star<T, D, H>(
    map: &[Vec<T>],
    start: (usize, usize),
    end: (usize, usize),
    dist: D,
    heur: H,
) -> Option<(Vec<(usize, usize)>, usize)>
where
    D: Fn((usize, usize), (usize, usize)) -> usize,
    H: Fn((usize, usize)) -> usize,
{
    let n = map.len();
    let m = map[0].len();

    let mut prevs: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut dists: HashMap<(usize, usize), usize> = HashMap::from([(start, 0)]);
    let mut to_visit: BTreeSet<(usize, (usize, usize))> = BTreeSet::from([(0, start)]);

    while let Some((_, mut curr)) = to_visit.pop_first() {
        if curr == end {
            let mut path = vec![curr];
            let dist = dists.get(&curr).unwrap();
            while curr != start {
                curr = *prevs.get(&curr).unwrap();
                path.push(curr);
            }
            return Some((path, *dist));
        }

        for neighbour in neighbours_bounded(curr, n, m) {
            let distance = dist(curr, neighbour) + *dists.get(&curr).unwrap();

            if *dists.get(&neighbour).unwrap_or(&usize::MAX) > distance {
                dists.insert(neighbour, distance);
                prevs.insert(neighbour, curr);
                to_visit.insert((distance + heur(neighbour), neighbour));
            }
        }
    }

    None
}
