#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    const PATH: &str = "day22/src/day22_input.txt";
    //part1(PATH);
    let start = Instant::now();
    part2(PATH);
    println!("{:?}", start.elapsed());
}
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
        .lines()
        .map(|n| sequence_bananas(n.trim().parse::<usize>().unwrap(), 2001))
        .collect::<Vec<_>>();

    let possible_sequences = prices.iter().flat_map(|n| n.keys()).collect::<HashSet<_>>();

    //println!("{}", possible_sequences.len());

    let mut max_seq = None;
    let mut max_val = 0;
    for seq in possible_sequences {
        let sum = prices
            .iter()
            .map(|p| p.get(seq).unwrap_or(&0))
            .sum::<usize>();
        if sum > max_val {
            max_val = sum;
            max_seq = Some(seq);
        }
    }

    println!("{max_seq:?} {max_val}");

    // let x = sequence_bananas(123, 10);
    // println!("{:?}", x);
}

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

fn evolve_n_times(mut secret: usize, n: usize) -> usize {
    for _ in 0..n {
        secret = evolve_bitwise(secret);
    }
    secret
}

fn sequence_bananas(secret: usize, n: usize) -> HashMap<Vec<isize>, usize> {
    assert!(n > 5,);

    let n1 = evolve_bitwise(secret);
    let n2 = evolve_bitwise(n1);
    let mut n3 = evolve_bitwise(n2);
    let mut n4 = evolve_bitwise(n3);

    let mut d1 = sequence_delta(secret % 10, n1 % 10);
    let mut d2 = sequence_delta(n1 % 10, n2 % 10);
    let mut d3 = sequence_delta(n2 % 10, n3 % 10);
    let mut d4 = sequence_delta(n3 % 10, n4 % 10);

    let mut map = HashMap::new();
    map.insert(vec![d1, d2, d3, d4], n4 % 10);

    for _ in 0..(n - 5) {
        n3 = n4;
        n4 = evolve_bitwise(n3);

        d1 = d2;
        d2 = d3;
        d3 = d4;
        d4 = sequence_delta(n3 % 10, n4 % 10);

        let key = vec![d1, d2, d3, d4];
        map.entry(key).or_insert_with(|| n4 % 10);
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
