# Advent of Code 2019 in Rust
Warm-up in anticipation for AoC21.

## Setup and use
Made to be run with `cargo` as an application. 
Copy the puzzle input into a text file in the folder `data`, with the filename `day#.txt`, replacing `#` with the relevant number.
To solve for that specific puzzle input, run:

```batch
cargo run --features day#
```

The answer together with execution time should be displayed.

To run all puzzles at the same time, run:
```batch
cargo run --features all
```

Once a puzzle has been completed, add the file `day#.txt` with each answer on seperate lines in the folder `data/default` to have it displayed without computing.
To toggle this feature, remove/add `"use_default"` inside the `default` feature, or specify is using the `--features use_default` argument when compiling.