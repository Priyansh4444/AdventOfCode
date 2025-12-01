# Advent of Code 2024

Here is a collection of my [Advent of Code](https://adventofcode.com/) submissions.

All of my solutions are written entirely in Rust, if I struggled I may have used the hyper-neutrino's solutions as a reference and marked it in the source code.

## Structure

Each year is in its own subdirectory under the root and is its own crate.

For the file layout, library files are contained in `src` and each day and its corresponding input is stored in `src/bin`. Each day is a two digit number representing the date plus the `.rs` extension. Each input is the same but with the `.txt` extension.

## Compiling

Ensure you have the Rust toolchain installed for your computer (visit [rustup.rs](https://rustup.rs) for more info).
Each day is its own binary application, using the date number to represent each day,
run either `cargo test --bin 13`to test day 13 for example. Or `cargo run --bin 13`
to run day 13.
