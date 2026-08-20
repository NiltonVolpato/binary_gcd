pub fn gcd_recursive(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_recursive(b, a % b)
}

pub fn gcd_iterative(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn gcd_binary_recursive(a: i64, b: i64) -> i64 {
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
        (false, false) => gcd_binary_recursive(a.abs_diff(b).cast_signed(), a.min(b)),
    }
}

pub fn gcd_binary(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let shift = a.trailing_zeros().min(b.trailing_zeros());
    a >>= shift;
    b >>= shift;

    loop {
        // Drop all factors of two from both operands, so neither is zero
        // and both are odd.
        a >>= a.trailing_zeros();
        b >>= b.trailing_zeros();
        // Keep a <= b so the difference below stays in [0, b).
        if a > b {
            let t = a;
            a = b;
            b = t;
        }
        let diff = b - a;
        if diff == 0 {
            // a == b, both odd and stripped: that's the gcd.
            return b << shift;
        }
        // b - a is even, so the next iteration's strip halves it.
        b = a;
        a = diff;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-verified expected values, including edge cases (zeros, equal
    /// inputs, one being a multiple of the other, and i64 extremes).
    const VECTORS: &[(i64, i64, i64)] = &[
        (0, 0, 0),
        (0, 5, 5),
        (5, 0, 5),
        (1, 1, 1),
        (1, 7, 1),
        (2, 2, 2),
        (3, 9, 3),
        // a has fewer factors of two than b.
        (6, 4, 2),
        (8, 12, 4),
        (5, 7, 1),
        (56, 98, 14),
        (1071, 462, 21),
        (100, 75, 25),
        // Consecutive values are always coprime.
        (i64::MAX, i64::MAX - 1, 1),
        // i64::MAX is odd.
        (i64::MAX, 2, 1),
        // 2^29 divides 2^30.
        (536_870_912, 1_073_741_824, 536_870_912),
        // Consecutive Fibonacci numbers: coprime and worst-case depth for the
        // Euclidean algorithm.
        (4_660_046_610_375_530_309, 7_540_113_804_746_346_429, 1), // F(91), F(92)
    ];

    macro_rules! known_values_test {
        ($test_name:ident, $func:path) => {
            #[test]
            fn $test_name() {
                for (i, &(a, b, expected)) in VECTORS.iter().enumerate() {
                    assert_eq!($func(a, b), expected, "vector #{i}: gcd({a}, {b})");
                }
            }
        };
    }

    known_values_test!(recursive_known_values, gcd_recursive);
    known_values_test!(iterative_known_values, gcd_iterative);
    known_values_test!(binary_recursive_known_values, gcd_binary_recursive);
    known_values_test!(binary_known_values, gcd_binary);

    /// Independent oracle (plain Euclid) for cross-checking random inputs.
    fn gcd_ref(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    macro_rules! random_cross_check {
        ($test_name:ident, $func:path) => {
            #[test]
            fn $test_name() {
                let mut state = 0x2545_F491_4F6C_DD1D; // splitmix64's recommended seed
                for i in 0..200_000u32 {
                    // Mix tiny values (hit zeros, ones and small GCDs), small
                    // values, and full-range values in [0, i64::MAX].
                    let kind = splitmix64(&mut state) % 4;
                    let a = match kind {
                        0 => (splitmix64(&mut state) % 8) as i64,
                        1 => (splitmix64(&mut state) % 1_000) as i64,
                        _ => (splitmix64(&mut state) & i64::MAX as u64) as i64,
                    };
                    let b = match kind {
                        0 => (splitmix64(&mut state) % 8) as i64,
                        1 => (splitmix64(&mut state) % 1_000) as i64,
                        _ => (splitmix64(&mut state) & i64::MAX as u64) as i64,
                    };
                    assert_eq!($func(a, b), gcd_ref(a, b), "pair #{i}: ({a}, {b})");
                }
            }
        };
    }

    /// Exhaustive small-range sweep: every (a, b) in [0, N)^2 against the
    /// reference, for each implementation. Fast; use to spot-check behavior
    /// before running the heavier random cross-checks.
    macro_rules! small_sweep_test {
        ($test_name:ident, $func:path) => {
            #[test]
            fn $test_name() {
                const N: i64 = 64;
                for a in 0..N {
                    for b in 0..N {
                        assert_eq!($func(a, b), gcd_ref(a, b), "pair ({a}, {b})");
                    }
                }
            }
        };
    }

    small_sweep_test!(recursive_small, gcd_recursive);
    small_sweep_test!(iterative_small, gcd_iterative);
    small_sweep_test!(binary_recursive_small, gcd_binary_recursive);
    small_sweep_test!(binary_small, gcd_binary);

    random_cross_check!(recursive_random, gcd_recursive);
    random_cross_check!(iterative_random, gcd_iterative);
    random_cross_check!(binary_recursive_random, gcd_binary_recursive);
    random_cross_check!(binary_random, gcd_binary);
}
