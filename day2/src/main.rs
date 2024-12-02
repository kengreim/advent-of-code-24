#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const PATH: &str = "day2/src/day2_input.txt";

    // Part 1
    let file = File::open(PATH).expect("file not found");
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines().map_while(Result::ok) {
        if is_safe_report(&parse_levels(&line)) {
            count += 1;
        }
    }
    println!("count = {count}");

    // Part 2
    let file2 = File::open(PATH).expect("file not found");
    let reader2 = BufReader::new(file2);

    let mut count2 = 0;
    for line in reader2.lines().map_while(Result::ok) {
        let levels = parse_levels(&line);
        if is_safe_report(&levels) {
            count2 += 1;
        } else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe_report(&new_levels) {
                    count2 += 1;
                    break;
                }
            }
        }
    }
    println!("count2 = {count2}");
}

fn parse_levels(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn is_safe_report(levels: &[i32]) -> bool {
    let mut is_increasing: Option<bool> = None;
    let windows = levels.windows(2);
    for window in windows {
        let delta = (window[1] - window[0]).abs();
        if !(1..=3).contains(&delta) {
            return false;
        }
        if let Some(increasing) = is_increasing {
            if (increasing && window[1] < window[0]) || (!increasing && window[0] < window[1]) {
                return false;
            }
        } else {
            is_increasing = Some(window[1] > window[0]);
        }
    }

    true
}
