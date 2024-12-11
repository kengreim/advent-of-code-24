#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::iter::zip;
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

    //let map = Arc::new(RwLock::new(HashMap::new()));

    // let sum = nums
    //     .iter()
    //     .map(|n| num_stones_2(VecDeque::from([]), *n, 1, 6, map.clone()))
    //     .sum::<i32>();
    // println!("Sum: {sum}");

    // let n = num_stones_2(
    //     17,
    //     VecDeque::from([17]),
    //     VecDeque::from([1]),
    //     5,
    //     map.clone(),
    // );

    let solves_map = Arc::new(RwLock::new(HashMap::new()));
    let max_iter = 75;

    let sum = nums
        .iter()
        .map(|n| {
            for i in 0..=max_iter {
                let sum = num_stones_2(*n, i, solves_map.clone());
                solves_map.write().unwrap().insert((*n, i), sum);
            }
            *solves_map.read().unwrap().get(&(*n, max_iter)).unwrap()
        })
        .sum::<i64>();
    //
    // for i in 1..=5 {
    //     let sum = num_stones(17, i);
    //     solves_map.write().unwrap().insert((17, i), sum);
    // }
    // println!("Sum: {}", solves_map.read().unwrap().get(&(17, 5)).unwrap());
    println!("Sum: {sum}");
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
    n: i64,
    iter_remaining: usize,
    solves_map: Arc<RwLock<HashMap<(i64, usize), i64>>>,
) -> i64 {
    if let Some(n) = solves_map.read().unwrap().get(&(n, iter_remaining)) {
        //println!("Found old solve for {n} with {iter_remaining} iters is {n}");
        return *n;
    }

    if iter_remaining == 0 {
        1
    } else {
        if n == 0 {
            let res = num_stones_2(1, iter_remaining - 1, solves_map.clone());
            solves_map.write().unwrap().insert((n, iter_remaining), res);
            res
        } else if n.to_string().len() % 2 == 0 {
            let (n1, n2) = split_num(n);
            let sum1 = num_stones_2(n1, iter_remaining - 1, solves_map.clone());
            let sum2 = num_stones_2(n2, iter_remaining - 1, solves_map.clone());
            solves_map
                .write()
                .unwrap()
                .insert((n, iter_remaining), sum1 + sum2);
            sum1 + sum2
        } else {
            let res = num_stones_2(n * 2024, iter_remaining - 1, solves_map.clone());
            solves_map.write().unwrap().insert((n, iter_remaining), res);
            res
        }
    }
}

//
// fn num_stones_2(
//     stone: i64,
//     mut previous_stones: VecDeque<i64>,
//     mut running_sums: VecDeque<i32>,
//     iter_remaining: usize,
//     sum_map: Arc<RwLock<HashMap<(i64, usize), i32>>>,
// ) -> i32 {
//     let read_lock = sum_map.read().unwrap();
//     let cached = read_lock.get(&(stone, iter_remaining)).copied();
//     drop(read_lock);
//
//     if iter_remaining == 0 {
//         //previous_stones.push_front(stone);
//         // for (iter, stone) in previous_stones.iter().enumerate().skip(1) {
//         //     //println!("Iter: {iter}: {stone}");
//         //     sum_map.write().unwrap().insert((*stone, iter), running_sum);
//         //     //println!("Sum: {stone}");
//         // }
//         1
//     } else if let Some(sum) = cached {
//         println!("found: sum {sum} for stone {stone} and {iter_remaining} iterations");
//         sum
//     } else {
//         if stone == 0 {
//             let mut lock = sum_map.write().unwrap();
//             println!("{:?}", previous_stones);
//             println!("{:?}", running_sums);
//
//             for (iter, stone) in previous_stones.iter().enumerate() {
//                 let sum = running_sums.iter().take(iter).sum::<i32>();
//                 lock.insert((*stone, iter + 1), sum);
//                 println!("1. Inserting: stone {stone} and {iter} iterations is sum {sum}",);
//             }
//
//             // for (iter, (stone, sum)) in zip(previous_stones.iter(), running_sums.iter()).enumerate()
//             // {
//             //     lock.insert((*stone, iter), *sum);
//             //     println!("1. Inserting: stone {stone} and {iter} iterations is sum {sum}",);
//             // }
//             drop(lock);
//
//             previous_stones.push_front(stone);
//             let new_sum = running_sums[0];
//             running_sums.push_back(new_sum);
//             num_stones_2(
//                 1,
//                 previous_stones.clone(),
//                 running_sums.clone(),
//                 iter_remaining - 1,
//                 sum_map.clone(),
//             )
//         } else if stone.to_string().len() % 2 == 0 {
//             let mut lock = sum_map.write().unwrap();
//             println!("{:?}", previous_stones);
//             for (iter, stone) in previous_stones.iter().enumerate() {
//                 let sum = running_sums.iter().take(iter).sum::<i32>();
//                 lock.insert((*stone, iter + 1), sum);
//                 println!("1. Inserting: stone {stone} and {iter} iterations is sum {sum}",);
//             }
//             drop(lock);
//
//             previous_stones.push_front(stone);
//             let (n1, n2) = split_num(stone);
//             let new_sum = running_sums[0] + 1;
//             running_sums.push_back(new_sum);
//             num_stones_2(
//                 n1,
//                 previous_stones.clone(),
//                 running_sums.clone(),
//                 iter_remaining - 1,
//                 sum_map.clone(),
//             ) + num_stones_2(
//                 n2,
//                 previous_stones.clone(),
//                 running_sums.clone(),
//                 iter_remaining - 1,
//                 sum_map.clone(),
//             )
//         } else {
//             let mut lock = sum_map.write().unwrap();
//             println!("{:?}", previous_stones);
//             for (iter, stone) in previous_stones.iter().enumerate() {
//                 let sum = running_sums.iter().take(iter).sum::<i32>();
//                 lock.insert((*stone, iter + 1), sum);
//                 println!("1. Inserting: stone {stone} and {iter} iterations is sum {sum}",);
//             }
//             drop(lock);
//
//             previous_stones.push_front(stone);
//             let new_sum = running_sums[0];
//             running_sums.push_back(new_sum);
//             num_stones_2(
//                 stone * 2024,
//                 previous_stones.clone(),
//                 running_sums.clone(),
//                 iter_remaining - 1,
//                 sum_map.clone(),
//             )
//         }
//     }
// }

fn split_num(n: i64) -> (i64, i64) {
    let s = n.to_string();
    let (s1, s2) = s.split_at(s.len() / 2);
    (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap())
}
