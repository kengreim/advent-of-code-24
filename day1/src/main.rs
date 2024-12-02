#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let file = File::open("day1/src/day1_input.txt").expect("file not found");

    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let splits = line.split_whitespace().collect::<Vec<_>>();
        list1.push(splits[0].parse::<i32>().unwrap());
        list2.push(splits[1].parse::<i32>().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    // Part 1
    let result = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("result: {result}");

    // Part 2
    let mut list1_map = HashMap::new();
    for i in &list1 {
        list1_map.insert(*i, 0);
    }

    for i in &list2 {
        list1_map.entry(*i).and_modify(|e| *e += 1);
    }

    let mut sum = 0;
    for i in &list1 {
        sum += *list1_map.entry(*i).or_default() * i;
    }
    println!("result: {sum}");
}
