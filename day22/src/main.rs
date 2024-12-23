#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs;
use std::time::Instant;

fn main() {
    const PATH: &str = "day22/src/day22_input.txt";
    //part1(PATH);
    let start = Instant::now();
    part2(PATH);
    println!("{:?}", start.elapsed());
}
#[allow(dead_code)]
fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let sum = input
        .lines()
        .map(|n| evolve_n_times(n.trim().parse::<usize>().unwrap(), 2000))
        .sum::<usize>();
    println!("{sum}");
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let prices = input
        .par_lines()
        .map(|n| sequence_bananas(n.trim().parse::<usize>().unwrap(), 2001))
        .collect::<Vec<_>>();

    let possible_sequences = prices
        .iter()
        .flat_map(|n| n.keys())
        .collect::<FxHashSet<_>>();

    //println!("{}", possible_sequences.len());

    let (max_seq, max_val) = possible_sequences
        .iter()
        .map(|&s| {
            (
                s,
                prices.iter().map(|p| p.get(s).unwrap_or(&0)).sum::<usize>(),
            )
        })
        .max_by_key(|&(_, s)| s)
        .unwrap();

    println!("{max_seq} {:?} {max_val}", deconstruct_key(*max_seq));
}

#[allow(dead_code)]
const fn evolve(n: usize) -> usize {
    let n1 = ((n * 64) ^ n) % 16777216;
    let n2 = ((n1 / 32) ^ n1) % 16777216;
    ((n2 * 2048) ^ n2) % 16777216
}

const fn evolve_bitwise(n: usize) -> usize {
    let n1 = ((n << 6) ^ n) & 16777215;
    let n2 = ((n1 >> 5) ^ n1) & 16777215;
    ((n2 << 11) ^ n2) & 16777215
}

#[allow(dead_code)]
fn evolve_n_times(secret: usize, n: usize) -> usize {
    (0..n).fold(secret, |acc, _| evolve_bitwise(acc))
}

fn sequence_bananas(secret: usize, n: usize) -> FxHashMap<isize, usize> {
    assert!(n > 5, "n must be greater than 5");

    let n1 = evolve_bitwise(secret);
    let n2 = evolve_bitwise(n1);
    let n3 = evolve_bitwise(n2);
    let mut n4 = evolve_bitwise(n3);

    let d1 = sequence_delta(secret % 10, n1 % 10);
    let d2 = sequence_delta(n1 % 10, n2 % 10);
    let d3 = sequence_delta(n2 % 10, n3 % 10);
    let mut d4 = sequence_delta(n3 % 10, n4 % 10);

    let mut last_price = n4 % 10;

    let mut map = FxHashMap::default();
    let mut key = make_key(d1, d2, d3, d4);
    map.insert(key, last_price);

    for _ in 0..(n - 5) {
        n4 = evolve_bitwise(n4);

        let new_price = n4 % 10;
        d4 = sequence_delta(last_price, new_price);
        last_price = new_price;

        key = ((key & 0x7FFF) << 5) | (d4 + 9);
        map.entry(key).or_insert_with(|| new_price);
    }

    map
}

const fn sequence_delta(n1: usize, n2: usize) -> isize {
    if n2 > n1 {
        (n2 - n1) as isize
    } else {
        -((n1 - n2) as isize)
    }
}

const fn make_key(n1: isize, n2: isize, n3: isize, n4: isize) -> isize {
    ((((n1 + 9) << 15) | ((n2 + 9) << 10)) | ((n3 + 9) << 5)) | (n4 + 9)
}

fn deconstruct_key(n: isize) -> Vec<isize> {
    let d1 = (n >> 15) - 9;
    let d2 = ((n >> 10) & 0b11111) - 9;
    let d3 = ((n >> 5) & 0b11111) - 9;
    let d4 = (n & 0b11111) - 9;
    vec![d1, d2, d3, d4]
}
