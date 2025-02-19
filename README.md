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

Keeping the project structure simple was great. Compared to previous years, this one was much simpler. Each daily solution is in its own module, and they all get included and run on `main()`, instead of having separate binaries and needing [a Bash script](https://github.com/epidemian/advent-of-code-2023/blob/main/run-all) to run all solutions like in 2023.

Tests for sample inputs are just unit tests on those same solution files, instead of having to glue the different binaries with their sample input files with [even more Bash](https://github.com/epidemian/advent-of-code-2023/blob/main/check-samples) like in 2023, or having [a complex `build.rs` file](https://github.com/epidemian/advent-of-code-2022/blob/main/build.rs) for auto-generating [unit tests for all sample files](https://github.com/epidemian/advent-of-code-2022/blob/main/src/sample_tests.rs) like in 2022.

Something else i learned from this year's AoC is to do less data modeling. There are no `struct` or `enum` declarations on these solutions. Thus, no `impl` blocks either. Maybe _some_ data could've been modeled with a custom struct (e.g. a common `Grid` type for grid-based puzzles), but i didn't really felt the need for that. Better to KISS.

### Day 1: Historian Hysteria

Simple puzzle to start the series. A nice excuse to learn about `Iterator::unzip()`, to collect an iterator of 2-element tuples into two containers.

But i also learned about the incredibly powerful `Itertools::process_results()` method, which allows to process an iterator of `Result` values with a function that is not `Result`-aware, while correctly propagating any errors up.

### Day 2: Red-Nosed Reports

Simple numeric puzzle, nothing crazy. A nice practice of polishing common patterns of parsing + data crunching.

### Day 3: Mull It Over

A weird one. Solved with regexes just because i didn't feel like doing the string parsing "manually".

### Day 4: Ceres Search

First grid puzzle of the season. Not an easy one to be only day 4! This felt like a nice exercise in trying to express a simple task (finding "XMAS" or the X-MAS crosses) in code in the most readable way, which was not too easy to do in Rust, given the usual bias of the language towards correctness instead of expressivity. I'm quite happy with the result :)

### Day 5: Print Queue

I had some fun trying to unify the common logic of part 1 and 2. (Part 1 only needed to check which sequences were sorted, and part 2 required to sort the unsorted sequences.)

### Day 6: Guard Gallivant

Another grid puzzle. My solution for part 2 feels super brute-forced (check all positions the guard walked on part 1, try out an obstacle in each one of them, and see if the guard gets stuck on a loop) and it's also very inefficient (>100ms), but at least it's simple enough to reason about.

### Day 7: Bridge Repair

A beautiful mathy puzzle. The solution was very "naturally recursive". And i also had some fun thinking about how to "concatenate" two numbers in base 10 without resorting to converting them to strings, concatenating those, and then parsing the result back to a number. Learned about the `ilog10()` method on integers.

### Day 8: Resonant Collinearity

Quite an interesting grid-based puzzle. It was a nice excuse of learning/practicing some iterator methods, like `Itertools::into_group_map()` for grouping values into a `HashMap`, or `Iterator::take_while()` for cutting-off the infinite iterators used to generate the antinode lines for part 2.

I also spent some time yak-shaving a generalized function for both part 1 and 2. This was more difficult than expected because the difference between part 1 and 2 revolves around how to generate antinodes, and *how many* antinodes are generated for a given pair of antennas. So the generalized function needs to take a closure for getting the antinodes for two given antennas, which is already a generic `Fn()` type, and then these closures need to return an arbitrary number of antinodes, which means they return different kinds of `Iterator`, which is a second level of generic types. This extra complexity could've easily been avoided by making the closures just return an owned type, e.g. `Vec<(i64, i64)>`, but it was more fun to find the most generic and allocation-avoiding solution :)

### Day 9: Disk Fragmenter

A typical AoC problem where part 2 becomes much more general and complicated than part 1. This time i decided to leave both parts as completely separate solutions, mostly because i found an elegant and simple way of implementing part 1 in terms of just swapping empty blocks (starting from the beginning of the disk) and used blocks (starting from the end of the disk).

### Day 10: Hoof It

First pathfinding problem of this season, and also another grid-walking puzzle. This one was rather easy thanks to the use of the `pathfinding` crate, which provides very general functions like `bfs_reach()` for finding all the possible trail ends for a given trailhead, or `count_paths()` for counting how many paths there are between a trailhead and an end.

### Day 11: Plutonian Pebbles

This was one of those typical AoC puzzles where part 1 can be solved naively, and then part 2 is just the same problem, but it bumps up a number that makes the naive solution completely infeasible. In this case, because both the time and space requirements of the naive solution grew exponentially: with 25 blinks we were on the ~100K simulated stones, a pretty manageable number, but with 75 blinks we were already on the ~100T stones, which would've required a long time to process, and more memory than i'll probably have access to in my lifetime.

Luckily the solution was a simple dynamic programming trick: treating stones as groups instead of individual things to simulate.

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


