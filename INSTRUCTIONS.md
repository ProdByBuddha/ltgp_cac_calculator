## Growth Quadrant Calculator Guide

This calculator classifies scenarios into growth quadrants using two simple boundaries derived from `CAC`, `CFA`, and `LTGP`.

### Boundaries
- Low vs High `CAC`: `CAC ≤ (low_cac_fraction × LTGP)`. Default `low_cac_fraction = 0.10` (10%).
- High vs Low `CFA`: `CFA ≥ 0.5 × CAC` is High CFA; otherwise Low CFA.

For any `LTGP = L` and threshold `t` (default `0.10`):

| Region    | Condition              |
|-----------|------------------------|
| Low CAC   | `0 < CAC ≤ t × L`      |
| High CAC  | `CAC > t × L`          |
| High CFA  | `CFA ≥ 0.5 × CAC`      |
| Low CFA   | `CFA < 0.5 × CAC`      |

### Quadrants (and what to enter)

Each quadrant is defined by the CAC and CFA regions:

- Self-Funding Growth (Low CAC, High CFA)
  - Conditions: `CAC ≤ t × LTGP` and `CFA ≥ 0.5 × CAC`

- Cash-Light Efficiency (Low CAC, Low CFA)
  - Conditions: `CAC ≤ t × LTGP` and `CFA < 0.5 × CAC`

- Deferred-Cash Risk (High CAC, High CFA)
  - Conditions: `CAC > t × LTGP` and `CFA ≥ 0.5 × CAC`

- Capital-Intensive Trap (High CAC, Low CFA)
  - Conditions: `CAC > t × LTGP` and `CFA < 0.5 × CAC`

### Quick Start (demo values)

Using `LTGP = 2500` and `t = 0.10` gives a CAC cut of `250`.

#### Self-Funding Growth
- Example: `CAC = 200 (≤ 250)`, `CFA = 200 (≥ 100)`
- Tip: Using `CFA = CAC` yields $0 net outlay and often an “Excellent” verdict.

```bash
cargo run -- --cac 200 --cfa 200 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

#### Cash-Light Efficiency
- Example: `CAC = 200 (≤ 250)`, `CFA = 80 (< 100)`

```bash
cargo run -- --cac 200 --cfa 80 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

#### Deferred-Cash Risk
- Example: `CAC = 400 (> 250)`, `CFA = 250 (≥ 200)`

```bash
cargo run -- --cac 400 --cfa 250 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

#### Capital-Intensive Trap
- Example: `CAC = 400 (> 250)`, `CFA = 150 (< 200)`

```bash
cargo run -- --cac 400 --cfa 150 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

### Interactive mode

Prefer entering values interactively? Supply the same numbers at the prompts.

- `Period` can be anything; it doesn’t affect the quadrant, only the payback display.
- `Early gross profit rate` doesn’t affect the quadrant either; it’s just for the payback estimate.

### Adapting to different LTGP or threshold

- Compute the CAC cut as `t × LTGP` (e.g., `LTGP = 10,000` with `t = 0.10` → `cut = 1,000`).
- Pick `CAC` on either side of that cut, then set `CFA` to above or below half of `CAC` to land in the desired quadrant.
