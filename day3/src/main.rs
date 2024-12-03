#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const PATH: &str = "day3/src/day3_input.txt";

    // Part 1
    let file = File::open(PATH).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        println!("{}", line)
    }
}
