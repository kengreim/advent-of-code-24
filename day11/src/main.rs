#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs;
use std::time::Instant;

fn main() {
    const PATH: &str = "day11/src/day11_input.txt";
    let start = Instant::now();
    part1(PATH);
    println!("Duration {:?}", start.elapsed());
}

fn part1(path: &str) {
    let nums = fs::read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let sum = nums.iter().map(|n| num_stones(*n, 25)).sum::<i64>();
    println!("Sum: {}", sum);
}

fn num_stones(n: i64, iter_remaining: i64) -> i64 {
    if iter_remaining == 0 {
        1
    } else {
        if n == 0 {
            num_stones(1, iter_remaining - 1)
        } else if n.to_string().len() % 2 == 0 {
            let (n1, n2) = split_num(n);
            num_stones(n1, iter_remaining - 1) + num_stones(n2, iter_remaining - 1)
        } else {
            num_stones(n * 2024, iter_remaining - 1)
        }
    }
}

fn split_num(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);
    (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap())
}
