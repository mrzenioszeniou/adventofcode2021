pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a.abs();
    }

    gcd(b, a % b)
}
