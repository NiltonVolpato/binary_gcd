use binary_gcd::{
    gcd_binary, gcd_binary_recursive, gcd_hybrid, gcd_iterative, gcd_recursive,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Consecutive Fibonacci numbers F(91), F(92): coprime, and the classic
/// worst case for the Euclidean algorithm (quotients are all 1).
const FIB: (i64, i64) = (4_660_046_610_375_530_309, 7_540_113_804_746_346_429);

/// Input sets to bench. `fib_worst_case` is the classic worst case for the
/// Euclidean algorithm (all quotients are 1); `max_odd` and `max_adjacent`
/// are worst cases for the binary algorithm, where a large odd operand
/// only halves about once per iteration (and the two 63-bit values in
/// `max_adjacent` stay close together the whole way down).
const INPUTS: [(&str, i64, i64); 5] = [
    ("small", 56, 98),
    ("medium", 1_071, 462),
    ("fib_worst_case", FIB.0, FIB.1),
    ("max_odd", i64::MAX, 2),
    ("max_adjacent", i64::MAX, i64::MAX - 1),
];

fn gcd_benchmarks(c: &mut Criterion) {
    for (label, a, b) in INPUTS {
        let recursive = format!("gcd/{label}/recursive");
        let iterative = format!("gcd/{label}/iterative");
        let binary_recursive = format!("gcd/{label}/binary_recursive");
        let binary = format!("gcd/{label}/binary");
        let hybrid = format!("gcd/{label}/hybrid");

        c.bench_function(&recursive, |bench| {
            bench.iter(|| gcd_recursive(black_box(a), black_box(b)))
        });
        c.bench_function(&iterative, |bench| {
            bench.iter(|| gcd_iterative(black_box(a), black_box(b)))
        });
        c.bench_function(&binary_recursive, |bench| {
            bench.iter(|| gcd_binary_recursive(black_box(a), black_box(b)))
        });
        c.bench_function(&binary, |bench| {
            bench.iter(|| gcd_binary(black_box(a), black_box(b)))
        });
        c.bench_function(&hybrid, |bench| {
            bench.iter(|| gcd_hybrid(black_box(a), black_box(b)))
        });
    }
}

criterion_group!(benches, gcd_benchmarks);
criterion_main!(benches);
