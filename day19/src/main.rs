#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rustc_hash::FxHashMap;
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
        .map(str::trim)
        .collect::<Vec<_>>();
    let designs = &lines[2..];

    let designs_memo = Arc::new(RwLock::new(
        towels
            .iter()
            .map(|&s| (s.to_owned(), true))
            .collect::<FxHashMap<String, bool>>(),
    ));

    let sum = designs
        .iter()
        .map(|&d| can_build_design(d, &designs_memo))
        .filter(|b| *b)
        .count();

    println!("{sum}");
}

fn can_build_design(design: &str, designs_memo: &Arc<RwLock<FxHashMap<String, bool>>>) -> bool {
    let previous_res = designs_memo.read().unwrap().get(design).copied();
    previous_res.map_or_else(
        || {
            let candidates = designs_memo
                .read()
                .unwrap()
                .keys()
                .filter_map(|s| {
                    if design.ends_with(s.as_str()) {
                        let new_len = design.len() - s.len();
                        Some(design[..new_len].to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let res = candidates
                .iter()
                .any(|left_str| can_build_design(left_str, designs_memo));

            designs_memo
                .write()
                .unwrap()
                .entry(design.to_string())
                .or_insert(res);

            res
        },
        |res| res,
    )
}
