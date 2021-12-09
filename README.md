# Advent of Code 2021 Solvers

## Tests

```bash
cargo test
```

## Run

```bash
cargo run --release -- day 1
```

You can pass multiple days and they will be run in the order provided:
```bash
cargo run --release -- day 1 2 3
```

Lowercase word representations are also accepted:
```bash
cargo run --release -- day one
```

## Benchmark

```bash
cargo bench
```

Normal `criterion` flags and filters work:
```bash
cargo bench --bench advent -- --list
cargo bench --bench advent -- "Day 1, Part Two"
```

## Development

### Adding solvers

Create a new module for your solvers (ie in `src/day_x.rs`). This module *must* contain:

* A function `part_one` of type `fn(&str) -> anyhow::Result<usize>`
* A function `part_two` of type `fn(&str) -> anyhow::Result<usize>`
* A `&str` `INPUT_PATH` that is the path to the puzzle input (generally `inputs/day/x/input`)

Expose this module in `lib.rs`, add a new match clause to `get_day`.

If you are adding an alternative solver for an existing day and want benchmarking, then you will need to update `bench/advent.rs`. `create_solvers_benchmark` should help.
