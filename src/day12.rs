use sscanf::scanf;
use std::collections::{HashMap, HashSet};

pub fn solve() -> (usize, usize) {
    let edges = parse(std::fs::read_to_string("res/day12.txt").unwrap());

    (solver(&edges, false), solver(&edges, true))
}

fn solver(edges: &HashMap<Cave, HashSet<Cave>>, double_take: bool) -> usize {
    let start = Cave::Small(String::from("start"));

    let paths: HashSet<Vec<Cave>> =
        traverse(&start, edges, HashSet::new(), double_take, "start", "end")
            .into_iter()
            .filter(|path| path[0].is("end"))
            .map(|mut p| {
                p.push(start.clone());
                p
            })
            .collect();

    paths.len()
}

fn traverse(
    curr: &Cave,
    edges: &HashMap<Cave, HashSet<Cave>>,
    mut hist: HashSet<Cave>,
    can_cheat: bool,
    start: &str,
    target: &str,
) -> Vec<Vec<Cave>> {
    if curr.is(target) {
        return vec![vec![]];
    }

    hist.insert(curr.clone());

    let mut paths = vec![];

    for to in edges.get(curr).unwrap() {
        let mut subpaths = if hist.contains(to) && to.is_small() {
            if can_cheat && !to.is(start) {
                traverse(to, edges, hist.clone(), false, start, target)
            } else {
                continue;
            }
        } else {
            traverse(to, edges, hist.clone(), can_cheat, start, target)
        };

        subpaths.iter_mut().for_each(|path| path.push(to.clone()));

        paths.append(&mut subpaths);
    }

    if paths.is_empty() {
        vec![vec![]]
    } else {
        paths
    }
}

fn parse(from: String) -> HashMap<Cave, HashSet<Cave>> {
    let mut ret = HashMap::new();

    for line in from.split('\n') {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let (from, to) = scanf!(trimmed, "{}-{}", String, String).unwrap();
        let from: Cave = from.into();
        let to: Cave = to.into();

        ret.entry(from.clone())
            .and_modify(|tos: &mut HashSet<Cave>| {
                tos.insert(to.clone());
            })
            .or_insert_with(|| HashSet::from([to.clone()]));

        ret.entry(to.clone())
            .and_modify(|tos: &mut HashSet<Cave>| {
                tos.insert(from.clone());
            })
            .or_insert_with(|| HashSet::from([from.clone()]));
    }

    ret
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Cave {
    Big(String),
    Small(String),
}

impl Cave {
    pub fn is(&self, name: &str) -> bool {
        match self {
            Self::Big(n) => n == name,
            Self::Small(n) => n == name,
        }
    }

    pub fn is_small(&self) -> bool {
        matches!(self, Self::Small(_))
    }
}

impl From<String> for Cave {
    fn from(from: String) -> Self {
        if from.chars().all(|c| c.is_uppercase()) {
            Self::Big(from)
        } else {
            Self::Small(from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::parse;

    #[test]
    fn example() {
        let example = r#"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "#;

        let edges = parse(example.to_string());

        assert_eq!(solver(&edges, false), 10);
        assert_eq!(solver(&edges, true), 36);
    }
}
