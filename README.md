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

This puzzle also provided another excuse for using the `ilog10()` method, which i learned about in day 7, for "splitting" numbers in half in base 10.

### Day 12: Garden Groups

This one had a lovely twist for part 2. My original approach of mixing up a flood-fill algorithm and the area and perimeter calculation, all in the same function, was terrible. I managed to copy and adapt it for part 2, but the result was awfully convoluted.

After getting both answers, i *had* to refactor that horrible code into something more palatable. At the very least, the logic of finding the regions on the map needed to be separated from the fence price calculation.

I used the <abbr title="Breadth-first search">BFS</abbr> algorithm from the `pathfinding` crate as a simple flood-filling solution for finding the regions. It was also nice to model the regions as simple sets of points. And after that, the calculations for the fence price ended up being at least a bit less gnarly to look at.

### Day 13: Claw Contraption

Part 1 could be solved naively, simply by iterating moves. But this didn't scale up for part 2, which was basically the but with giant numbers. Luckily, it wasn't too hard to find a direct analytical solution doing a bit of maths.

It was nice to think of a way doing this sort of equation-solving using only integers and not needed to foray into floating-point territory :)

### Day 14: Restroom Redoubt

An absolute delight of an unexpected Easter egg on part 2. Beware of looking at the code without having done this puzzle, lest you get spoiled!

### Day 15: Warehouse Woes

Nice Sokoban-inspired puzzle. Both part 1 and 2 were great. Though part 2 was not easy at all, with those pesky 2-cell-wide boxes to push around.

Once again, the BFS algorithm from `pathfinding` came super handy. This time for collecting all boxes that needed to be moved by a robot push.

Also, modelling the terrain as a map of 2D points to "things" (`HashMap<(i64, i64), char>`), instead of the typical 2D grid, simplified the code a bit. And it also made it easier to not crash in cases of bad inputs due to out-of-bounds indexing.

### Day 16: Reindeer Maze

A very Dijkstra-friendly part 1 (thanks again, `pathfidning` crate!). But a very tricky part 2, which was sort of similar to a Dijkstra thing, but at the same time totally different, since it needed to find *all* possible best paths and not just one.

I first found the answer to part 2 using a terribly inefficient approach of Dijkstra-ing from all possible tiles to see which ones were part of a best path. It worked, but it took a couple of minutes to run; well outside the runtime goal.

I ended up implementing two efficient-enough solutions. One which involves [finding tiles that "join" with the best](./src/day_16_reindeer_maze.rs) path with the same cost, and in that way finding all the other alternative best paths. And the other one implements [a custom Dijkstra-like algorithm](./src/day_16_reindeer_maze_custom_dijkstra.rs) that finds all the paths with the minimum cost instead of a single one.

### Day 17: Chronospatial Computer

A very tricky part 2. I became super suspicious of small inputs after this one — a lesson i probably should have learned from previous AoC's.

Once i figured out what the simulated program was doing, i ended up finding correct input number using, surprisingly, BFS. Yet again!

Part 2 ended up having more lines dedicated to [a long explanatory comment](./src/day_17_chronospatial_computer.rs) than to code.

### Day 18: RAM Run

This one was kind of breather compared with the previous ones. I felt quite proud for having found an excuse to use binary search for quickly finding the number of bytes that needed to have fallen in order to block the path. I learned that the function `slice::partition_point()` can be used for exactly that purpose: at which point does this predicate stop being true?

### Day 19: Linen Layout

Part 1 could be cheesed by using regexes. But part 2 required a bit more thought. Luckily, the combinatorial counting could be expressed quite naturally with a recursive function, and then it was a matter of adding some caching to get a non-eternal runtime.

### Day 20: Race Condition

An easy pathfinding 2-grid problem with an interesting twist. It was quite fun trying to come up with a general solution for both parts 1 and 2. At first, part 2 seemed daunting, but thinking in terms of [Taxicab distances](https://en.wikipedia.org/wiki/Taxicab_geometry) from the cells in the shortest path to cells which we can "cheat" into made it much more tractable.

### Day 21: Keypad Conundrum

Probably the hardest puzzle of the year. Short inputs really mean hard puzzles! Part 1 was already mind-bending enough, with its "meta" feeling of directional keypads being controlled by other directional keypads and so on, and my first solution for it was some of the scrappiest code i've ever written.

But then part 2 was the usual ramp-up of numbers that makes a naive solution infeasible in terms of runtime or memory consumption, due to the exponential nature of the problem. It required some clever decomposition and, as usual, caching.

This is the only problem where i felt so stuck that i went looking for "inspiration" online. Making the general length-counting function recursive and cached by robot level is an idea i ~~found on~~ stole from reddit.

### Day 22: Monkey Market

Quite a simple puzzle for day 22. Part 1 is a straightforward pseudo-random number generation problem. And for part 2, i was initially overthinking things and considering that i had to simulate the buying process for all possible price difference sequences. But it could be solved in a straightforward manner by just adding up the different banana amounts the buying monkey would buy while the difference sequences are being generated.

Also, this was a nice example of when imperative code can be simpler and read better than the decomposed iterator-based approach.

### Day 23: LAN Party

First graph problem of the year. They are usually hard so the come up in the last days. Part 1 was simple enough to do in a naive way: count pair combination for computers that are connected to computers whose name start with "t".

Part would've required some fancy graph algorithm, like last year's [day 25 puzzle](https://github.com/epidemian/advent-of-code-2023?tab=readme-ov-file#day-25-snowverload), but luckily this is a known problem and the `pathfinding` crate has an implementation of an algorithm to [find all the complete sub-graphs](https://docs.rs/pathfinding/latest/pathfinding/undirected/cliques/index.html), known as cliques. So i just re-used that :)

### Day 24: Crossed Wires

This was sort of another graph puzzle, but didn't feel much like it. Part 1 was a simple-enough circuit simulation. I went for a backtracking approach, starting from the end cables and "going back" to the initial inputs to generate the output bits.

Then part 2 was one of the problems where you have to think about the shape of the particular input — instead of *any* general input — and find and ad-hoc solution there. It took me some time to come up with a code solution that was not horribly ugly.

### Day 25: Code Chronicle

A super easy ending. It even seemed like the problem description was a red herring to over-complicate the solution: the number of matching key-lock pairs could be found by simply comparing their string representation and checking that no "#" overlap.

I guess this was a nice breather for Christmas. Although i ended up solving it on Easter, which is... fitting, in a way.

