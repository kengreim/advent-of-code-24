#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rustc_hash::FxHashMap;
use std::fs;
use std::sync::{Arc, RwLock};

fn main() {
    const PATH: &str = "day19/src/day19_input_test.txt";
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
        .map(str::trim)
        .collect::<Vec<_>>();
    let designs = &lines[2..];

    let memo_set = Arc::new(RwLock::new(
        towels
            .iter()
            .map(|&s| (s.to_owned(), true))
            .collect::<FxHashMap<String, bool>>(),
    ));

    let mut sum = 0;
    for &d in designs {
        //println!("try {d}");
        if can_build_design(d, "", memo_set.clone()) {
            sum += 1;
        }
    }

    // let sum = designs
    //     .iter()
    //     .map(|&d| can_build_design(d, &towels, memo_set.clone()))
    //     .filter(|b| *b)
    //     .count();

    println!("{sum}");
}

fn can_build_design(
    design: &str,
    remainder: &str,
    towels_memo: Arc<RwLock<FxHashMap<String, bool>>>,
) -> bool {
    let previous_res = towels_memo.read().unwrap().get(design).cloned();
    if let Some(res) = previous_res {
        //memo_set.write().unwrap().insert(design.to_string());
        //println!("found {design} in memo");
        res
    } else {
        // println!(
        //     "{:?}",
        //     towels_memo.read().unwrap().keys().collect::<Vec<_>>()
        // );
        let candidates = towels_memo
            .read()
            .unwrap()
            .keys()
            .filter_map(|s| {
                //println!("trying {}", s);
                if design.ends_with(s.as_str()) {
                    let new_len = design.len() - s.len();
                    let remainder = format!("{}{}", design[new_len..].to_owned(), remainder);
                    //println!("ends with {s} in memo");
                    Some((design[..new_len].to_owned(), remainder))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        //println! {"{:?}", candidates};

        let mut write_lock = towels_memo.write().unwrap();
        for (left_str, remainder) in &candidates {
            //println!("candidate {left_str} and writing remainder {remainder}");

            write_lock.entry(remainder.to_owned()).or_insert(true);

            //write_lock.insert(remainder.clone());
        }
        drop(write_lock);

        //println!("done");

        //println!("{design} {:?}", candidates);

        let res = candidates.iter().any(|(left_str, remainder)| {
            //println!("call can build {left_str} {remainder}");
            can_build_design(left_str, remainder, towels_memo.clone())
        });
        towels_memo
            .write()
            .unwrap()
            .entry(design.to_string())
            .or_insert(res);
        //
        // if res {
        //     println!("writing {design}");
        // }

        res
    }
}
