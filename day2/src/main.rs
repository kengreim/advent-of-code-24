use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day2/src/day2_input.txt").expect("file not found");

    let reader = BufReader::new(file);

    // Part 1
    let mut count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if is_safe_report(&levels) {
                count += 1;
            }
        }
    }
    println!("count = {}", count);

    // Part 2
    let file2 = File::open("day2/src/day2_input.txt").expect("file not found");
    let reader2 = BufReader::new(file2);

    let mut count2 = 0;
    for line in reader2.lines() {
        if let Ok(line) = line {
            let levels = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
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
    }
    println!("count2 = {}", count2);
}

fn is_safe_report(levels: &[i32]) -> bool {
    let mut is_increasing: Option<bool> = None;
    let windows = levels.windows(2);
    for window in windows {
        let delta = (window[1] - window[0]).abs();
        if delta > 3 || delta < 1 {
            return false;
        }
        if let Some(increasing) = is_increasing {
            if (increasing && window[1] < window[0]) || (!increasing && window[0] < window[1]) {
                return false;
            }
        } else {
            is_increasing = if window[1] > window[0] {
                Some(true)
            } else {
                Some(false)
            }
        }
    }

    true
}
