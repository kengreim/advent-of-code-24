#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use regex::Regex;
use std::fs;

fn main() {
    const PATH: &str = "day3/src/day3_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let sum = re.captures_iter(&input).fold(0, |acc, capture| {
        acc + capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
    });
    println!("Sum: {sum}");

    // Part 2
    let re2 = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut enabled = true;
    let mut sum2 = 0;
    for capture in re2.captures_iter(&input) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                sum2 += enabled as i32
                    * &capture[1].parse::<i32>().unwrap()
                    * &capture[2].parse::<i32>().unwrap()
            }
        }
    }
    println!("Sum2: {sum2}");

    // Part 2 functional
    let re2 = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();
    let (sum3, _) = re2
        .captures_iter(&input)
        .fold((0, true), |(sum, enabled), capture| match &capture[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ => (
                sum + enabled as i32
                    * &capture[1].parse::<i32>().unwrap()
                    * &capture[2].parse::<i32>().unwrap(),
                enabled,
            ),
        });
    println!("Sum3: {}", sum3);
}
