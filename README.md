# binary_gcd

Five GCD implementations for `i64`, with a test suite, criterion benchmarks, a plotting script, and a comparison against the popular [`gcd`](https://crates.io/crates/gcd) crate.

## Implementations (`src/lib.rs`)

| function | approach |
| --- | --- |
| `gcd_recursive` | recursive Euclidean algorithm |
| `gcd_iterative` | iterative Euclidean algorithm (modulo based) |
| `gcd_binary_recursive` | recursive binary GCD (Stein's) |
| `gcd_binary` | iterative binary GCD, with an early exit once the running minimum reaches 1 (`gcd(k, 1) == 1`); the loop terminates on `diff == 0` |
| `gcd_hybrid` | Lehmer-style hybrid: binary (subtract + shift) steps when quotients are small, a single modulo when the bit lengths differ by 3+ (quotient ≥ 4) |

All return `gcd(a, b)`, and each strips common factors of two up front.

## Tests (`cargo test`, 15 tests)

For every implementation:

- **known values** — hand-verified vectors, including zeros, equal inputs, multiples, `i64::MAX` pairs, and consecutive Fibonacci numbers (F(91), F(92));
- **exhaustive small sweep** — all 4,096 pairs in `[0, 64)²` against an independent Euclid reference;
- **random cross-check** — 200,000 seeded (splitmix64) pairs mixing tiny, small, and full-range values against the same reference.

## Benchmarks (`cargo bench`, criterion)

Input sets benched: `small (56, 98)`, `medium (1071, 462)`, `fib_worst_case (F(91), F(92))`, `max_odd (i64::MAX, 2)`, `max_adjacent (i64::MAX, i64::MAX - 1)`.

The external comparison uses `gcd = "2.3"` via its `Gcd` trait; because that trait is only implemented for unsigned integers, the `i64` inputs are cast to `u64` for the `gcd_crate` column.

Latest run (median, ns):

| input | recursive | iterative | binary_recursive | binary | hybrid | gcd_crate |
| --- | --- | --- | --- | --- | --- | --- |
| small | 2.040 | 2.039 | 4.916 | 1.502 | 1.732 | **1.371** |
| medium | 1.734 | **1.539** | 6.977 | 2.252 | 3.018 | 2.215 |
| fib_worst_case | 204.596 | 260.199 | 74.292 | **33.646** | 63.634 | 38.319 |
| max_odd | 1.487 | **1.047** | 82.392 | 1.245 | 1.683 | 62.273 |
| max_adjacent | 1.490 | **1.048** | 114.374 | 1.494 | 2.223 | 62.292 |

Machine: Apple M4, 10-core (macOS, 2026-10-14).

`gcd_binary` is the implementation that matters: it is statistically tied with or faster than the `gcd` crate on every input, and it dominates on the binary-GCD worst cases (`max_odd` and `max_adjacent`), where the early-exit once the running minimum reaches 1 makes it roughly **50× faster** than `gcd_crate`. It is also ~6× faster than the Euclidean variants on the Fibonacci worst case.

![Median time per input set](benchmarks.png)

## Plotting

```sh
./.venv/bin/python scripts/plot_benchmarks.py benchmarks.png
```

Reads `target/criterion/gcd_*/new/estimates.json` (from the latest `cargo bench`) and renders a log-scale grouped bar chart. Add new implementation names to `IMPLS` in the script when the crate grows.

## Quick start

```sh
cargo test      # run the test suite
cargo bench     # run the criterion benchmarks
```
