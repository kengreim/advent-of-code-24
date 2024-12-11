#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::{Arc, RwLock};
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

fn part2(path: &str) {
    let nums = fs::read_to_string(path)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let sum = nums.iter().map(|n| num_stones_2(*n, 25)).sum::<i64>();
    println!("Sum: {}", sum);
}

fn num_stones(n: i64, iter_remaining: i32) -> i64 {
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

fn num_stones_2(
    mut previous_stones: VecDeque<i64>,
    stone: i64,
    running_sum: i64,
    iter_remaining: i32,
    sum_map: Arc<RwLock<HashMap<(i64, i32), i32>>>,
) -> i32 {
    if iter_remaining == 0 {
        previous_stones.push_front(stone);
        for (iter, stone) in previous_stones.iter().enumerate() {
            sum_map
                .write()
                .unwrap()
                .insert((*stone, *iter), *running_sum)
        }
        1
    } else if let Some(sum) = sum_map.read().unwrap().get(&(stone, iter_remaining)) {
        *sum
    } else {
        if stone == 0 {
            previous_stones.push_front(stone);
            for (iter, stone) in previous_stones.iter().enumerate() {
                sum_map
                    .write()
                    .unwrap()
                    .insert((*stone, *iter), *running_sum)
            }

            num_stones_2(
                previous_stones.clone(),
                1,
                running_sum,
                iter_remaining - 1,
                Arc::clone(&sum_map),
            )
        } else if stone.to_string().len() % 2 == 0 {
            previous_stones.push_front(stone);
            for (iter, stone) in previous_stones.iter().enumerate() {
                sum_map
                    .write()
                    .unwrap()
                    .insert((*stone, *iter), *running_sum + 1)
            }

            let (n1, n2) = split_num(stone);
            num_stones_2(
                previous_stones.clone(),
                n1,
                running_sum + 1,
                iter_remaining - 1,
                Arc::clone(&sum_map),
            ) + num_stones_2(
                previous_stones.clone(),
                n2,
                running_sum + 1,
                iter_remaining - 1,
                Arc::clone(&sum_map),
            )
        } else {
            for (iter, stone) in previous_stones.iter().enumerate() {
                sum_map
                    .write()
                    .unwrap()
                    .insert((*stone, *iter), *running_sum)
            }

            num_stones_2(
                previous_stones.clone(),
                stone * 2024,
                running_sum,
                iter_remaining - 1,
                Arc::clone(&sum_map),
            )
        }
    }
}

fn split_num(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);
    (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap())
}
