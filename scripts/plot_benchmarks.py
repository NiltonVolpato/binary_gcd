#!/usr/bin/env python3
"""Plot criterion benchmark results for the GCD implementations.

Reads target/criterion/gcd_*/new/estimates.json and renders a grouped
bar chart (one bar per implementation per input, log scale) to a PNG.

Usage: ./.venv/bin/python scripts/plot_benchmarks.py [output.png]
"""

import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

CRITERION_DIR = Path("target/criterion")

# The bench target emits gcd/{input}/{impl} per criterion directory, where
# the input label may itself contain underscores (e.g. fib_worst_case).
IMPLS = ["recursive", "iterative", "binary_recursive", "binary", "hybrid", "gcd_crate"]


def split_bench_name(name):
    """gcd_{input}_{impl} -> (input, impl), matching the impl from the end."""
    base = name.removeprefix("gcd_")
    for impl in sorted(IMPLS, key=len, reverse=True):
        if base.endswith(f"_{impl}"):
            return base[: -len(impl) - 1], impl
    raise ValueError(f"unrecognized benchmark directory: {name}")


def collect_results():
    """Return {input_label: {impl: median_ns}} from the latest run."""
    results = {}
    for path in sorted(CRITERION_DIR.glob("gcd_*/new/estimates.json")):
        input_label, impl = split_bench_name(path.parent.parent.name)
        with open(path) as f:
            est = json.load(f)["median"]["point_estimate"]
        results.setdefault(input_label, {})[impl] = est
    return results


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("benchmarks.png")

    results = collect_results()
    if not results:
        sys.exit(f"No criterion results found under {CRITERION_DIR.resolve()} — run `cargo bench` first")

    inputs = sorted(results, key=lambda k: max(results[k].values()))
    impls = sorted({impl for group in results.values() for impl in group})
    colors = plt.cm.tab10(np.linspace(0, 1, len(impls)))

    n_groups = len(inputs)
    n_bars = len(impls)
    positions = np.arange(n_groups)
    width = 0.8 / n_bars

    fig, ax = plt.subplots(figsize=(12, 6))
    for i, impl in enumerate(impls):
        values = [results[inp].get(impl, float("nan")) for inp in inputs]
        ax.bar(positions + (i - (n_bars - 1) / 2) * width, values, width,
               label=impl, color=colors[i])

    ax.set_yscale("log")
    ax.set_xlabel("benchmark input")
    ax.set_ylabel("median time (ns, log scale)")
    ax.set_title("GCD implementations — median time per input set (latest criterion run)")
    ax.set_xticks(positions, inputs, rotation=15, ha="right")
    ax.legend()
    ax.grid(axis="y", which="both", linestyle=":", alpha=0.4)
    fig.tight_layout()
    fig.savefig(out)
    print(f"wrote {out}")

    # Text summary too, so the plot isn't the only record of the numbers.
    print(f"\n{'input':<16} " + " ".join(f"{i:<19}" for i in impls))
    for inp in inputs:
        print(f"{inp:<16} " + " ".join(
            f"{results[inp].get(impl, float('nan')):<19.3f}" for impl in impls))


if __name__ == "__main__":
    main()
