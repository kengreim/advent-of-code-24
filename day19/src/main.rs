#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rustc_hash::FxHashMap;
use std::cmp::max;
use std::fs;
use std::time::Instant;

fn main() {
    const PATH: &str = "day19/src/day19_input.txt";

    let start = Instant::now();
    //part1(PATH);
    part1_with_p2(PATH);
    //part2(PATH);
    println!("{:?}", start.elapsed());
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (towels, designs) = parse_input(&input);

    let mut designs_memo = towels
        .iter()
        .map(|&s| (s.to_owned(), true))
        .collect::<FxHashMap<String, bool>>();

    let sum = designs
        .iter()
        .map(|&d| u32::from(can_build_design(d, &mut designs_memo)))
        .sum::<u32>();

    println!("{sum}");
}

fn part1_with_p2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (towels, designs) = parse_input(&input);

    let mut designs_memo = FxHashMap::default();
    let sum = designs
        .iter()
        .filter_map(|&d| num_ways_build_design(d, &towels, &mut designs_memo))
        .filter(|&n| n > 0)
        .count();
    println!("{sum}");
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (towels, designs) = parse_input(&input);

    let mut designs_memo = FxHashMap::default();
    let sum = designs
        .iter()
        .map(|&d| num_ways_build_design(d, &towels, &mut designs_memo).unwrap_or_default())
        .sum::<u64>();
    println!("{sum}");

    // let answers = designs
    //     .iter()
    //     .map(|&d| {
    //         (
    //             d,
    //             num_ways_build_design(d, &towels, &mut designs_memo).unwrap_or_default(),
    //         )
    //     })
    //     .collect::<Vec<(&str, u64)>>();
    // let sum = answers.iter().map(|&(_, n)| n).sum::<u64>();
    //println!("{answers:?}");
}

fn can_build_design(design: &str, designs_memo: &mut FxHashMap<String, bool>) -> bool {
    let previous_res = designs_memo.get(design).copied();
    previous_res.map_or_else(
        || {
            let mut keys = designs_memo
                .iter()
                .filter_map(|(k, &v)| if v { Some(k) } else { None })
                .collect::<Vec<_>>();
            keys.sort_by_key(|b| std::cmp::Reverse(b.len()));

            let candidates = keys
                .into_iter()
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

            designs_memo.entry(design.to_string()).or_insert(res);

            res
        },
        |res| res,
    )
}

fn num_ways_build_design(
    design: &str,
    original_towels: &[&str],
    fn_memo: &mut FxHashMap<String, u64>,
) -> Option<u64> {
    let previous_res = fn_memo.get(design).copied();
    previous_res.map_or_else(
        || {
            if design.is_empty() {
                return Some(1);
            }

            let candidates = original_towels
                .iter()
                .filter_map(|s| {
                    if design.ends_with(s) {
                        let new_len = design.len() - s.len();
                        Some(design[..new_len].to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if candidates.is_empty() {
                //println!("No ways to make {design}");
                return None;
            }

            let branch_sum = candidates
                .iter()
                .filter_map(|left_str| num_ways_build_design(left_str, original_towels, fn_memo))
                .sum::<u64>();

            //println!("There are {branch_sum} ways to make {design}");

            fn_memo
                .entry(design.to_string())
                .and_modify(|existing| {
                    *existing = max(branch_sum, *existing);
                })
                .or_insert(branch_sum);

            Some(branch_sum)
        },
        |res| {
            //println!("Found previous result for {design}: {res}");
            Some(res)
        },
    )
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = input.lines().collect::<Vec<_>>();
    let towels = lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    let designs = &lines[2..];
    (towels, designs.to_vec())
}
