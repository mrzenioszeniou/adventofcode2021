use std::collections::{HashMap, HashSet};

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
