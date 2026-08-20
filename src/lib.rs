pub fn gcd_recursive(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_recursive(b, a % b)
}

pub fn gcd_iterative(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_binary_recursive(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, b) => return b,
        (a, 0) => return a,
        (a, b) if a == b => return a,
        _ => {}
    };

    match (a % 2 == 0, b % 2 == 0) {
        (true, true) => 2 * gcd_binary_recursive(a / 2, b / 2),
        (true, false) => gcd_binary_recursive(a / 2, b),
        (false, true) => gcd_binary_recursive(a, b / 2),
        (false, false) => gcd_binary_recursive(a.abs_diff(b), a.min(b)),
    }
}

pub fn gcd_binary(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let mut az = a.trailing_zeros();
    let bz = b.trailing_zeros();
    let shift = az.min(bz);
    b >>= shift;

    loop {
        a >>= az;
        let diff = b - a;
        az = diff.trailing_zeros();
        b = a.min(b);
        a = a.abs_diff(0);
        if a == 0 {
            return b << shift;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive() {
        let result = gcd_recursive(2, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn iterative() {
        let result = gcd_iterative(2, 2);
        assert_eq!(result, 2);
    }
}
