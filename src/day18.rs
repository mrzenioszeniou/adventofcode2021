pub fn solve() -> (usize, usize) {
    (42, 42)
}

fn split(sequence: &mut Vec<char>) -> bool {
    for i in 0..sequence.len() - 1 {
        if let Some((num, n)) = parse_num(&sequence[i..]) {
            if num > 9 {
                let left = num.unstable_div_floor(2);
                let right = num.unstable_div_ceil(2);

                sequence.splice(i..i + n, format!("[{},{}]", left, right).chars());

                return true;
            }
        }
    }

    false
}

fn explode(sequence: &mut Vec<char>) -> bool {
    let mut depth = 0;

    for i in 0..sequence.len() {
        if depth >= 4 {
            if let Some((left, right)) = parse_pair(&sequence[i..i + 5]) {
                println!("\n\n{}", String::from_iter(sequence.clone()));
                sequence.splice(i..i + 5, ['0']);
                println!("{}", String::from_iter(sequence.clone()));

                // Add right
                for j in i + 1..sequence.len() {
                    if let Some((num, n)) = parse_num(&sequence[j..]) {
                        sequence.splice(j..j + n, (num + right).to_string().chars());
                        break;
                    }
                }
                println!("{}", String::from_iter(sequence.clone()));

                // Add left
                for j in (0..i).rev() {
                    if let Some((num, n)) = parse_num(&sequence[j..]) {
                        sequence.splice(j..j + n, (num + left).to_string().chars());
                        break;
                    }
                }

                println!("{}", String::from_iter(sequence.clone()));

                return true;
            }
        }

        match sequence[i] {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => {}
        }
    }

    false
}

fn parse_num(from: &[char]) -> Option<(u32, usize)> {
    let s: String = from.iter().take_while(|c| c.is_numeric()).collect();

    s.parse().ok().map(|num| (num, s.len()))
}

fn parse_pair(sequence: &[char]) -> Option<(u32, u32)> {
    if sequence[0] != '[' {
        return None;
    }

    let (left, n) = parse_num(&sequence[1..])?;

    if sequence[n + 1] != ',' {
        return None;
    }

    let (right, m) = parse_num(&sequence[n + 2..])?;

    if sequence[n + m + 2] == ']' {
        Some((left, right))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn examples() {
        let mut s = "[[[[[9,8],1],2],3],4]".chars().collect();
        assert!(explode(&mut s));
        assert!(s.into_iter().eq("[[[[0,9],2],3],4]".chars()));

        let mut s = "[7,[6,[5,[4,[3,2]]]]]".chars().collect();
        assert!(explode(&mut s));
        assert!(s.into_iter().eq("[7,[6,[5,[7,0]]]]".chars()));

        let mut s = "[[6,[5,[4,[3,2]]]],1]".chars().collect();
        assert!(explode(&mut s));
        assert!(s.into_iter().eq("[[6,[5,[7,0]]],3]".chars()));

        let mut s = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars().collect();
        assert!(explode(&mut s));
        assert!(s
            .into_iter()
            .eq("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars()));

        let mut s = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars().collect();
        assert!(explode(&mut s));
        assert!(s.into_iter().eq("[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars()));

        let mut s = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".chars().collect();
        assert!(explode(&mut s));
        assert_eq!(String::from_iter(&s), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        assert!(explode(&mut s));
        assert_eq!(String::from_iter(&s), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        assert!(!explode(&mut s));
        assert!(split(&mut s));
        assert_eq!(String::from_iter(&s), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        assert!(!explode(&mut s));
        assert!(split(&mut s));
        assert_eq!(
            String::from_iter(&s),
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
        );

        assert!(explode(&mut s));
        assert_eq!(String::from_iter(&s), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }
}
