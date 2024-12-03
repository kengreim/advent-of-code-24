#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs;

fn main() {
    const PATH: &str = "day4/src/day4_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();
}
