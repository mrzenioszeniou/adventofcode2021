use std::{num::ParseIntError, ops::Deref, str::FromStr};

pub fn solve() -> (usize, usize) {
    (42, 42)
}

#[derive(Debug, PartialEq, Eq)]
enum Number {
    Value(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Self::Value(v) => *v,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn add_left(&mut self, n: u32) {
        match self {
            Self::Value(v) => *v += n,
            Self::Pair(left, _) => left.add_left(n),
        }
    }

    fn add_right(&mut self, n: u32) {
        match self {
            Self::Value(v) => *v += n,
            Self::Pair(_, right) => right.add_right(n),
        }
    }

    pub fn explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Self::Value(_) => None,
            Self::Pair(left, right) => {
                if depth >= 4 && left.is_value() && right.is_value() {
                    let ret = Some((Some(left.magnitude()), Some(right.magnitude())));
                    *self = Self::Value(0);
                    ret
                } else if let Some((ret_left, ret_right)) = left.explode(depth + 1) {
                    if ret_right.is_some() && right.is_value() {
                        *right = Box::new(Self::Value(right.magnitude() + ret_right.unwrap()));
                        Some((ret_left, None))
                    } else {
                        Some((ret_left, ret_right))
                    }
                } else if let Some((ret_left, ret_right)) = right.explode(depth + 1) {
                    if ret_left.is_some() && left.is_value() {
                        *left = Box::new(Self::Value(left.magnitude() + ret_left.unwrap()));
                        Some((None, ret_right))
                    } else {
                        Some((ret_left, ret_right))
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Self::Value(v) => v.to_string(),
            Self::Pair(l, r) => format!("[{},{}]", l.to_string(), r.to_string()),
        }
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut queue = vec![];

        for c in s.chars() {
            if c == '[' || c == ',' {
                continue;
            }

            if c == ']' {
                let right = Box::new(queue.pop().unwrap());
                let left = Box::new(queue.pop().unwrap());
                queue.push(Self::Pair(left, right));
            } else {
                let value = c.to_digit(10).unwrap();
                queue.push(Self::Value(value));
            }
        }

        Ok(queue.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!("4".parse(), Ok(Number::Value(4)));
        assert_eq!(
            "[1,2]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Value(1)),
                Box::new(Number::Value(2))
            ))
        );

        assert_eq!(
            "[[1,2],3]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Value(1)),
                    Box::new(Number::Value(2))
                )),
                Box::new(Number::Value(3))
            ))
        );

        assert_eq!(
            "[9,[8,7]]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Value(9)),
                Box::new(Number::Pair(
                    Box::new(Number::Value(8)),
                    Box::new(Number::Value(7))
                )),
            ))
        );

        assert_eq!(
            "[[1,9],[8,5]]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Value(1)),
                    Box::new(Number::Value(9))
                )),
                Box::new(Number::Pair(
                    Box::new(Number::Value(8)),
                    Box::new(Number::Value(5))
                )),
            ))
        );

        assert_eq!(
            "[[[[[9,8],1],2],3],4]".parse(),
            Ok(Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Pair(
                        Box::new(Number::Pair(
                            Box::new(Number::Pair(
                                Box::new(Number::Value(9)),
                                Box::new(Number::Value(8))
                            )),
                            Box::new(Number::Value(1))
                        )),
                        Box::new(Number::Value(2))
                    )),
                    Box::new(Number::Value(3))
                )),
                Box::new(Number::Value(4))
            )),
        );

        let mut n: Number = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        n.explode(0);
        assert_eq!(n.to_string(), "[[[[0,9],2],3],4]");

        let mut n: Number = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        n.explode(0);
        assert_eq!(n.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut n: Number = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        n.explode(0);
        assert_eq!(n.to_string(), "[[6,[5,[7,0]]],3]");

        // let mut n: Number = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
        // n.explode(0);
        // assert_eq!(n.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        // [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action).
        // [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]].
    }
}
