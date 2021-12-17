pub fn solve() -> (isize, isize) {
    (part1(248, 285, -85, -56), part2(248, 285, -85, -56))
}

fn check(
    mut vel_x: isize,
    mut vel_y: isize,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    short: bool,
) -> Option<isize> {
    let mut max_height = 0;
    let mut x = 0;
    let mut y = 0;
    let mut landed = false;

    while !(x > max_x || x < min_x && vel_x == 0 || y < min_y && vel_y < 0) {
        if max_height < y {
            max_height = y;
        }

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            landed = true;
            if short {
                return Some(max_height);
            }
        }

        x += vel_x;
        y += vel_y;
        if vel_x > 0 {
            vel_x -= 1;
        }
        vel_y -= 1;
    }

    if landed {
        Some(max_height)
    } else {
        None
    }
}

fn part1(min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> isize {
    let mut record = 0;

    for vel_x in 0..=max_x {
        for vel_y in std::cmp::min(min_y, 0)..100 {
            if let Some(height) = check(vel_x, vel_y, min_x, max_x, min_y, max_y, false) {
                record = std::cmp::max(record, height);
            }
        }
    }

    record
}

fn part2(min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> isize {
    let mut cnt = 0;

    for vel_x in 0..=max_x {
        for vel_y in std::cmp::min(min_y, 0)..100 {
            if check(vel_x, vel_y, min_x, max_x, min_y, max_y, true).is_some() {
                cnt += 1;
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(part1(20, 30, -10, -5), 45);
        assert_eq!(part2(20, 30, -10, -5), 112);
    }
}
