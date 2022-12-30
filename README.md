# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 | Notes |
| :---: | :---: | :---:  | :---: |
| [Day 1](./day1/src/main.rs) | `1.2ms` | `1.2ms` | |
| [Day 2](./day2/src/main.rs) | `1.3ms` | `1.6ms` | |
| [Day 3](./day3/src/main.rs) | `1.7ms` | `1.6ms` | |
| [Day 4](./day4/src/main.rs) | `1.6ms` | `1.4ms` | |
| [Day 5](./day5/src/main.rs) | `2.4ms` | `1.7ms` | |
| [Day 6](./day6/src/main.rs) | `0.9ms` | `1.7ms` | HashSets |
| [Day 7](./day7/src/main.rs) | `1.2ms` | `1.6ms` | Tree using [id_tree](https://docs.rs/id_tree/latest/id_tree/) |
| [Day 8](./day8/src/main.rs) | `16.7ms` | `30.1s` |Dataframes using [Polars](https://www.pola.rs/) |

**Total: 67.9ms**
<!--- benchmarking table --->

Benchmarks run using:
 ```bash
 hyperfine --runs 100  -N  "target/release/day{n} 1"
 hyperfine --runs 100  -N  "target/release/day{n} 2"
 ```

They include file IO which likely dominates the benchmarking times for the first week or so.
