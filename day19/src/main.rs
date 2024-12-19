#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, RwLock};

fn main() {
    const PATH: &str = "day19/src/day19_input.txt";
    part1(PATH);
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let towels = lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();
    let designs = &lines[2..];

    let memo_set = Arc::new(RwLock::new(HashSet::<String>::new()));
    let sum = designs
        .iter()
        .map(|&d| can_build_design(d, &towels, memo_set.clone()))
        .filter(|b| *b)
        .count();

    println!("{sum}");
}

fn can_build_design(design: &str, towels: &[&str], memo_set: Arc<RwLock<HashSet<String>>>) -> bool {
    if memo_set.read().unwrap().contains(design) {
        return true;
    }

    if towels.iter().any(|t| t == &design) {
        memo_set.write().unwrap().insert(design.to_string());
        true
    } else {
        let candidates = towels
            .iter()
            .filter_map(|&s| {
                //println!("trying {}", s);
                if design.ends_with(s) {
                    let new_len = design.len() - s.len();
                    Some((design[..new_len].to_owned(), s))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        //println!("{design} {:?}", candidates);

        let res = candidates
            .iter()
            .any(|(remaining, _)| can_build_design(remaining, towels, memo_set.clone()));

        if res {
            memo_set.write().unwrap().insert(design.to_string());
        }

        res
    }
}
