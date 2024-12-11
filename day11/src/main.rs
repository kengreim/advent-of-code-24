#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use std::time::Instant;

fn main() {
    const PATH: &str = "day11/src/day11_input.txt";
    let start = Instant::now();
    //part1(PATH);
    part2(PATH);
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

    let sum = nums.iter().map(|n| num_stones(*n, 6)).sum::<i64>();
    println!("Sum: {sum}");
}

fn part2(path: &str) {
    let nums = fs::read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let solves_map = Arc::new(RwLock::new(HashMap::new()));
    let max_iter = 75;

    let sum = nums
        .iter()
        .map(|n| {
            for i in 0..=max_iter {
                let sum = num_stones_memoize(*n, i, solves_map.clone());
                solves_map.write().unwrap().insert((*n, i), sum);
            }
            *solves_map.read().unwrap().get(&(*n, max_iter)).unwrap()
        })
        .sum::<i64>();

    println!("Sum: {sum}");
}

fn num_stones(n: i64, iter_remaining: i32) -> i64 {
    if iter_remaining == 0 {
        1
    } else if n == 0 {
        num_stones(1, iter_remaining - 1)
    } else if n.to_string().len() % 2 == 0 {
        let (n1, n2) = split_num(n);
        num_stones(n1, iter_remaining - 1) + num_stones(n2, iter_remaining - 1)
    } else {
        num_stones(n * 2024, iter_remaining - 1)
    }
}

fn num_stones_memoize(
    n: i64,
    iter_remaining: usize,
    solves_map: Arc<RwLock<HashMap<(i64, usize), i64>>>,
) -> i64 {
    if let Some(n) = solves_map.read().unwrap().get(&(n, iter_remaining)) {
        return *n;
    }

    if iter_remaining == 0 {
        1
    } else if n == 0 {
        let res = num_stones_memoize(1, iter_remaining - 1, solves_map.clone());
        solves_map.write().unwrap().insert((n, iter_remaining), res);
        res
    } else if n.to_string().len() % 2 == 0 {
        let (n1, n2) = split_num(n);
        let sum1 = num_stones_memoize(n1, iter_remaining - 1, solves_map.clone());
        let sum2 = num_stones_memoize(n2, iter_remaining - 1, solves_map.clone());
        solves_map
            .write()
            .unwrap()
            .insert((n, iter_remaining), sum1 + sum2);
        sum1 + sum2
    } else {
        let res = num_stones_memoize(n * 2024, iter_remaining - 1, solves_map.clone());
        solves_map.write().unwrap().insert((n, iter_remaining), res);
        res
    }
}

fn split_num(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);
    (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap())

    // let d = 10i64.pow(s.len() as u32 / 2) as i64;
    // let (div, rem) = (n / d, n % d);
    // (div, rem)
}
