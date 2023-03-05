# Reginae - nQueens analyzer

This is yet another tool to solve arbitrary [nQueens](https://en.wikipedia.org/wiki/Eight_queens_puzzle) boards. This is a classic computer science problem and was studied by the brightest minds; from Gauss to Dijkstra.

The goal of this implementation is to help the analysis of picked heuristics with some debugging capabilities. We allow dynamic injection of evaluators with their weight.

We *don't* use conventional artificial intelligence techniques that will randomly mutate the decision making. Instead, we try to tackle the problem with a completely deterministic set of evaluators that will score for every iteration of the path. The idea is to evaluate how the jumps behaves for deterministic algorithms. We have many amazing evolutionary/genetic algorithms out there, and it shouldn't be hard to find these implementations if you're looking for such approaches.

#### Solution

The `Solver` uses a [A-star](https://en.wikipedia.org/wiki/A*_search_algorithm) approach, so it will simply take the highest score and deplete that path, either achieving a solution or blacklisting all the rotations of the board.

The blacklist can easily become massive, so we might want to revisit this idea. A couple of things can be done to mitigate this, such as truncating a path once it has been completely depleted. This is feasible, it's just not implemented.

We currently use a radix tree to mitigate the memory cost, but we still might have some trouble if the heuristics for the A* is not good enough as we will end up blacklisting a massive amount of paths (easily gigabytes of memory). If you see the resources of your system going to space, just `CTRL+C` :)

The implementation *can* be full no-std and run in any embedded system. It will all be a matter of performance, as we will need a good trie implementation that will run for that target system.

#### Custom evaluator injection

The function must be declared with `#[no_mangle]` with signature `fn(&Board, usize) -> f64`. Check `./evaluators/src/lib.rs`. This implementation is `#![no_std]`, but that isn't required.

It will take the current state of the board, the last move, and it expects a `f64` between `0.0` and `1.0`. The higher the value, the hight the priority of this board for the execution path.

The crate should be set to `dylib`. Check `./evaluators/Cargo.toml`. `dylib` is the standard format used by Rust. `cdylib` should, theoretically, work as well.

#### Example

It's not really important to use release mode as the main culprit is the memory space; every computation is very cheap, being just some operations with a bunch of integers. But, naturally, the performance will be far better if optimized for release.

This will build not only our binary endpoints, but also the shared objects that we will inject at runtime.
```shell
cargo build --release
```

You can play around with the terminal UI implementation, but currently there is no command to inject the heuristics (however, there is no technical limitation for that, it's just not implemented). The interface shows Vim bindings to navigate around, but arrows will work as well.
```shell
cargo run --release --bin reginae-tui
```

To inject heuristics with the CLI endpoint
```shell
echo 12 | cargo run --release --bin reginae-cli -- \
  -l target/release/libreginae_evaluators.so:overlapping:10 \
  -l target/release/libreginae_evaluators.so:ladder:5 \
  -l target/release/libreginae_evaluators.so:wrapping_ladder:-5
```

The command above will solve a 12x12 board.

It will read the arguments from stdin, separated by comma `,`, and the first element will be the width of the board. The remainder elements will be indexes of queens to be preset to the board. It will increment, sequentially, from the top-left of the board, until the bottom-right.

A queen positioned at the `c7` coordinate of a regular width 8 chess board will be represented as `10`, while a queen at `b8` will be `1`.

The `-l` argument will inject heuristics into the execution. The format must be `path:function:weight`. The weight is optional, and will be parsed as `1.0` if omitted.
