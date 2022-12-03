# Advent of Code 2022 (Rust)

[![codecov](https://codecov.io/gh/davidlag0/advent-of-code-2022/branch/main/graph/badge.svg?token=YBGR2fclvo)](https://codecov.io/gh/davidlag0/advent-of-code-2022)
[![Rust CI](https://github.com/davidlag0/advent-of-code-2022/actions/workflows/rust.yml/badge.svg)](https://github.com/davidlag0/advent-of-code-2022/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This year (2022), I'm doing Advent of Code again with Rust! As I go through the puzzles, I will attempt the challenges on my own and then improve it by looking at other people's code.

## Usage

### Solve puzzles
```sh
$ cargo run <path to folder with input files>
```

### Run tests in current environment
```sh
$ cargo test -- --nocapture
```

## Development

### Prepare Environment
```sh
$ make init
```

### Code coverage (Reference: https://github.com/mozilla/grcov)
```sh
$ make coverage
```
