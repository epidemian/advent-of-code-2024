# Advent of Code 2024

My solutions for [Advent of Code 2024](https://adventofcode.com/2024).

Some of these solves were recorded and uploaded [to YouTube](https://www.youtube.com/playlist?list=PL3kymB6hDjyUS7bB6rD_0-TLyqtVLIh_4).

## Goals

- Have fun & learn things
- Fast execution time (< 1 second for whole set of puzzles)
- Proper error propagation. Avoid crashes on bad inputs
- Write not-to-terrible code. Try to make things readable and succinct.

## Running stuff

Assuming you have Rust installed, you can run all solutions using `cargo run`, or run a single day solution passing the day number: `cargo run 17`. You may also pass the `--release` or `-r` flag to run things on release mode, which is much faster.

To run unit tests (which mainly check that the sample inputs produce the expected outputs) use `cargo test`, or pass in the day number to run the tests for that single day: `cargo test 08` (this is using Cargo's built-in fuzzy search to match test names, so 08 is passed instead of 8 to avoid also running the tests for day 18).

The `./check-all` script runs all tests and checks all solutions against the answers in [`answers.txt`](./answers.txt).

## Notes & Learnings

### Day 1: Historian Hysteria

Simple puzzle to start the series. A nice excuse to learn about `Iterator::unzip()`, to collect an iterator of 2-element tuples into two containers.

But i also learned about the incredibly powerful `Itertools::process_results()` method, which allows to process an iterator of `Result` values with a function that is not `Result`-aware, while correctly propagating any errors up.

### Day 2: Red-Nosed Reports

### Day 3: Mull It Over

### Day 4: Ceres Search

### Day 5: Print Queue

### Day 6: Guard Gallivant

### Day 7: Bridge Repair

### Day 8: Resonant Collinearity

### Day 9: Disk Fragmenter

### Day 10: Hoof It

### Day 11: Plutonian Pebbles

### Day 12: Garden Groups

### Day 13: Claw Contraption

### Day 14: Restroom Redoubt

### Day 15: Warehouse Woes

### Day 16: Reindeer Maze

### Day 17: Chronospatial Computer

### Day 18: RAM Run

### Day 19: Linen Layout

### Day 20: Race Condition

### Day 21: Keypad Conundrum

### Day 22: Monkey Market

### Day 23: LAN Party

### Day 24: Crossed Wires

### Day 25: Code Chronicle


