#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::{Arc, RwLock};
use std::time::Instant;

fn main() {
    const PATH: &str = "day11/src/day11_example.txt";
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

    let map = Arc::new(RwLock::new(HashMap::new()));

    // let sum = nums
    //     .iter()
    //     .map(|n| num_stones_2(VecDeque::from([]), *n, 0, 6, map.clone()))
    //     .sum::<i32>();

    let n = num_stones_2(VecDeque::from([]), 125, 1, 6, map.clone());
    println!("Sum: {}", n);
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
    running_sum: i32,
    iter_remaining: usize,
    sum_map: Arc<RwLock<HashMap<(i64, usize), i32>>>,
) -> i32 {
    let mut read_lock = sum_map.read().unwrap();
    let cached = read_lock.get(&(stone, iter_remaining)).copied();
    drop(read_lock);

    if iter_remaining == 0 {
        previous_stones.push_front(stone);
        for (iter, stone) in previous_stones.iter().enumerate() {
            //println!("Iter: {iter}: {stone}");
            sum_map.write().unwrap().insert((*stone, iter), running_sum);
            //println!("Sum: {stone}");
        }
        running_sum
    } else if let Some(sum) = cached {
        println!("found: {sum}");
        sum
    } else {
        previous_stones.push_front(stone);

        if stone == 0 {
            let mut lock = sum_map.write().unwrap();
            for (iter, stone) in previous_stones.iter().enumerate() {
                lock.insert((*stone, iter), running_sum);
            }
            drop(lock);

            num_stones_2(
                previous_stones.clone(),
                1,
                running_sum,
                iter_remaining - 1,
                sum_map.clone(),
            )
        } else if stone.to_string().len() % 2 == 0 {
            let mut lock = sum_map.write().unwrap();
            for (iter, stone) in previous_stones.iter().enumerate() {
                lock.insert((*stone, iter), running_sum + 1);
            }
            drop(lock);

            let (n1, n2) = split_num(stone);
            num_stones_2(
                previous_stones.clone(),
                n1,
                running_sum + 1,
                iter_remaining - 1,
                sum_map.clone(),
            ) + num_stones_2(
                previous_stones.clone(),
                n2,
                running_sum + 1,
                iter_remaining - 1,
                sum_map.clone(),
            )
        } else {
            let mut lock = sum_map.write().unwrap();
            for (iter, stone) in previous_stones.iter().enumerate() {
                lock.insert((*stone, iter), running_sum);
            }
            drop(lock);

            num_stones_2(
                previous_stones.clone(),
                stone * 2024,
                running_sum,
                iter_remaining - 1,
                sum_map.clone(),
            )
        }
    }
}

fn split_num(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);
    (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap())
}
