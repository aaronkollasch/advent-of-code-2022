# Advent of Code 2022 in Rust
## Links
- https://github.com/timvisee/advent-of-code-2022

## Timings

| Day | Part A    | Part B    |
|-----|----------:|----------:|
| 1   | 10.59 µs  | 11.61 μs  |
| 2   | 7.298 μs  | 7.318 μs  |
| 3   | 18.20 μs  | 16.47 μs  |
| 4   | 40.64 μs  | 39.69 μs  |
| 5   | 18.98 μs  | 20.44 μs  |
| 6   | 1.892 μs  | 618.1 ns  |
| 7   | 6.706 μs  | 6.772 μs  |
| 8   | 30.04 μs  | 201.3 μs  |
| 9   | 26.72 μs  | 97.07 μs  |
| 10  | 617.0 ns  | 835.2 ns  |
| 11  | 4.608 μs  | 3.054 ms  |
| 12  | 153.5 μs  | 102.7 μs  |
| 13  | 6.981 μs  | 9.667 μs  |
| 14  | 28.08 μs  | 69.15 μs  |
| 15  | 3.421 μs  | 3.538 μs  |
| 16  | 14.87 ms  | 16.15 ms  |
| 17  | 57.01 μs  | 218.3 μs  |
| 18  | 154.0 μs  | 599.3 μs  |
| 19  | 161.6 μs  | 187.7 μs  |
| 20  | 2.337 ms  | 64.80 ms  |
| 21  | 226.1 μs  | 1.668 ms  |
| 22  | 70.41 μs  | 5.238 ms  |
| 23  | 1.326 ms  | 83.06 ms  |
| 24  | 223.3 μs  | 598.3 μs  |
| 25  | 1.451 μs  |           |

Total runtime: 193.90 ms

## How to Run

First copy the input text files into the `inputs/` directory, and name them by day, e.g. `day01.txt`.

To run an individual solution in debug mode:
```shell
cargo run --bin day01a
```

To run everything in parallel:
```shell
cargo run --release --bin runner-par
```

To benchmark every day:
```shell
cargo run --release --bin bench
```

or using criterion:
```shell
cargo bench --bench all-days
```

To run tests:
```shell
cargo test --lib
```

## Other years

- [2025](https://github.com/aaronkollasch/advent-of-code-2025)
- [2022](https://github.com/aaronkollasch/advent-of-code-2022) _(current)_

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
