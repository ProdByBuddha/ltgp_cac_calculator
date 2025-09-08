# Crow's Nest — LTGP:CAC Growth Economics Calculator

Crow's Nest is a lightweight, guided CLI that turns growth economics into clear, cash-safe decisions. It asks for your CAC, CFA, LTGP, and early gross profit in plain language—explaining what each metric is, where to get it, why it matters, and who it applies to—then classifies your model and estimates payback.

Think of it like the crow’s nest on a ship: it climbs above the waves and calls out “safe passage” vs “reef ahead,” so you can steer with confidence.

## Features
- Interactive guided form (use `--interactive` or run with missing args)
- Clear explanations for every input: what, where/how to get it, why it matters, who it applies to
- Quadrant classification: Self-Funding Growth, Cash-Light Efficiency, Deferred-Cash Risk, Capital-Intensive Trap
- LTGP:CAC ratio, net upfront outlay, and plain-English verdict
- Payback period estimate in days/weeks/months/years

## Quick start

Clone and run locally:

```bash
# build and run the guided form
cargo run -- --interactive

# or run non-interactively with flags
cargo run -- --cac 500 --cfa 200 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

To install the binary locally from this repo:

```bash
cargo install --path .
```

## Usage

Interactive (recommended the first time):

```bash
cargo run -- --interactive
```

Non-interactive (provide all values as flags):

```bash
cargo run -- --cac 200 --cfa 150 --ltgp 2500 --early-gp-rate 50 --period days --low-cac-fraction 0.10
```

What you’ll get:
- Net cash outlay (CAC − CFA)
- LTGP:CAC ratio
- Quadrant label and a plain-language verdict
- Payback period estimate (in your chosen unit)

## Ship analogy (why this matters)
- CAC is the headwind. CFA is wind in your sails. LTGP is the value of your cargo. Early gross profit is your speed. Payback is the distance to the next safe harbor.
- Crow’s Nest gives forward visibility so you can avoid “capital‑intensive traps,” protect runway, and choose safer, faster growth routes.

## Roadmap
- Save and compare scenarios
- Visual quadrant charts and payback timelines
- Simple web UI on top of the CLI

## Contributing
Issues and PRs are welcome. If you want to discuss features or integrations, open an issue.

## License
MIT License — see LICENSE for details.

