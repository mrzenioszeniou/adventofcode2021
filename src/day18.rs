pub fn solve() -> (u32, usize) {
    let list: Vec<String> = std::fs::read_to_string("res/day18.txt")
        .unwrap()
        .split('\n')
        .map(|l| l.to_string())
        .collect();

    (part1(&list), 42)
}

fn part1(list: &[String]) -> u32 {
    let mut curr: Vec<_> = list[0].chars().collect();

    for next in list.iter().skip(1) {
        curr.insert(0, '[');
        curr.push(',');
        next.chars().for_each(|c| curr.push(c));
        curr.push(']');
        print!("Processing {} .. ", String::from_iter(curr.iter()));

        while explode(&mut curr) || split(&mut curr) {}
        println!("OK");
    }

    magnitude(&curr)
}

fn magnitude(sequence: &[char]) -> u32 {
    let mut queue = vec![];

    for c in sequence {
        if *c == ']' {
            let right = queue.pop().unwrap();
            let left = queue.pop().unwrap();

            queue.push(3 * left + 2 * right);
        } else if c.is_numeric() {
            queue.push(c.to_digit(10).unwrap());
        }
    }

    assert_eq!(queue.len(), 1);

    queue.pop().unwrap()
}

fn split(sequence: &mut Vec<char>) -> bool {
    for i in 0..sequence.len() - 1 {
        if let Some((num, n)) = parse_num_right(&sequence[i..]) {
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
            if let Some((left, right, n)) = parse_pair(&sequence[i..]) {
                println!("\n\n{}", String::from_iter(sequence.clone()));
                sequence.splice(i..i + n, ['0']);
                println!("{}", String::from_iter(sequence.clone()));

                // Add right
                for j in i + 1..sequence.len() {
                    if let Some((num, n)) = parse_num_right(&sequence[j..]) {
                        sequence.splice(j..j + n, (num + right).to_string().chars());
                        break;
                    }
                }
                println!("{}", String::from_iter(sequence.clone()));

                // Add left
                for j in (0..i).rev() {
                    if let Some((num, n)) = parse_num_left(&sequence[0..=j]) {
                        sequence.splice(j - n + 1..=j, (num + left).to_string().chars());
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

fn parse_num_left(from: &[char]) -> Option<(u32, usize)> {
    let mut s = vec![];

    for c in from.iter().rev() {
        if c.is_numeric() {
            s.push(*c);
        } else {
            break;
        }
    }

    s.reverse();

    String::from_iter(&s).parse().ok().map(|num| (num, s.len()))
}

fn parse_num_right(from: &[char]) -> Option<(u32, usize)> {
    let s: String = from.iter().take_while(|c| c.is_numeric()).collect();

    s.parse().ok().map(|num| (num, s.len()))
}

fn parse_pair(sequence: &[char]) -> Option<(u32, u32, usize)> {
    if sequence[0] != '[' {
        return None;
    }

    let (left, n) = parse_num_right(&sequence[1..])?;

    if sequence[n + 1] != ',' {
        return None;
    }

    let (right, m) = parse_num_right(&sequence[n + 2..])?;

    if sequence[n + m + 2] == ']' {
        Some((left, right, n + m + 3))
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

        assert!(!explode(&mut s));
        assert!(!split(&mut s));

        assert_eq!(
            magnitude(&"[[1,2],[[3,4],5]]".chars().collect::<Vec<_>>()),
            143
        );
        assert_eq!(
            magnitude(
                &"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            1384
        );
        assert_eq!(
            magnitude(&"[[[[1,1],[2,2]],[3,3]],[4,4]]".chars().collect::<Vec<_>>()),
            445
        );
        assert_eq!(
            magnitude(&"[[[[3,0],[5,3]],[4,4]],[5,5]]".chars().collect::<Vec<_>>()),
            791
        );
        assert_eq!(
            magnitude(&"[[[[5,0],[7,4]],[5,5]],[6,6]]".chars().collect::<Vec<_>>()),
            1137
        );
        assert_eq!(
            magnitude(
                &"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                    .chars()
                    .collect::<Vec<_>>()
            ),
            3488
        );

        let list: Vec<_> = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#
            .split('\n')
            .map(|l| l.trim().to_string())
            .collect();

        assert_eq!(part1(&list), 4140);
    }
}
