use std::collections::HashSet;

pub fn solve() -> (usize, usize) {
    let (image, algorithm) = parse(&std::fs::read_to_string("res/day20.txt").unwrap());

    (
        solver(&image, &algorithm, 2),
        solver(&image, &algorithm, 50),
    )
}

fn solver(
    image: &HashSet<(isize, isize)>,
    algorithm: &HashSet<usize>,
    enhancements: usize,
) -> usize {
    let mut image = image.clone();
    let mut edge = Edge::new(false);

    for _ in 0..enhancements {
        (image, edge) = enhance(&image, algorithm, edge);
    }

    image.len()
}

fn enhance(
    image: &HashSet<(isize, isize)>,
    algorithm: &HashSet<usize>,
    gb: Edge,
) -> (HashSet<(isize, isize)>, Edge) {
    let (min_i, max_i, min_j, max_j) = edges(image);

    let mut enhanced_image = HashSet::new();

    for pixel_i in min_i - 1..=max_i + 1 {
        for pixel_j in min_j - 1..=max_j + 1 {
            let mut bits = String::with_capacity(9);

            for i in pixel_i - 1..pixel_i + 2 {
                for j in pixel_j - 1..pixel_j + 2 {
                    let bit = if i < min_i || i > max_i || j < min_j || j > max_j {
                        gb.bit()
                    } else if image.contains(&(i, j)) {
                        '1'
                    } else {
                        '0'
                    };

                    bits.push(bit);
                }
            }

            let index = usize::from_str_radix(&bits, 2).unwrap();

            if algorithm.contains(&index) {
                enhanced_image.insert((pixel_i, pixel_j));
            }
        }
    }

    (enhanced_image, Edge::new(algorithm.contains(&gb.index())))
}

fn edges(image: &HashSet<(isize, isize)>) -> (isize, isize, isize, isize) {
    let mut min_i = isize::MAX;
    let mut max_i = isize::MIN;
    let mut min_j = isize::MAX;
    let mut max_j = isize::MIN;

    for pixel in image.iter() {
        min_i = std::cmp::min(min_i, pixel.0);
        max_i = std::cmp::max(max_i, pixel.0);
        min_j = std::cmp::min(min_j, pixel.1);
        max_j = std::cmp::max(max_j, pixel.1);
    }

    (min_i, max_i, min_j, max_j)
}

fn parse(from: &str) -> (HashSet<(isize, isize)>, HashSet<usize>) {
    let mut split = from.split('\n');

    let algorithm = split
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let mut image = HashSet::new();

    for (i, line) in split.enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    image.insert((i as isize, j as isize));
                }
                '.' => {}
                e => panic!("Unexpected character encountered '{}' in image", e),
            }
        }
    }

    (image, algorithm)
}

#[derive(Debug, Default, Clone, Copy)]
struct Edge {
    is_lit: bool,
}

impl Edge {
    pub fn new(is_lit: bool) -> Self {
        Self { is_lit }
    }

    pub fn bit(&self) -> char {
        if self.is_lit {
            '1'
        } else {
            '0'
        }
    }

    pub fn index(&self) -> usize {
        if self.is_lit {
            511
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn examples() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

        let (image, algorithm) = parse(input);

        assert_eq!(solver(&image, &algorithm, 2), 35);
        assert_eq!(solver(&image, &algorithm, 50), 3351);
    }
}
