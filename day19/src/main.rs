#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rustc_hash::FxHashMap;
use std::cmp::max;
use std::fs;
use std::time::Instant;

fn main() {
    const PATH: &str = "day19/src/day19_input.txt";
    //part1(PATH);
    let start = Instant::now();
    part2(PATH);
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
        .map(|&d| can_build_design(d, &mut designs_memo) as u32)
        .sum::<u32>();

    println!("{sum}");
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (towels, designs) = parse_input(&input);
    //let designs_memo = RwLock::new(towels.iter().map(|&s| (s.to_owned(), 0)).collect());

    let mut designs_memo = FxHashMap::default();
    let answers = designs
        .iter()
        .map(|&d| {
            (
                d,
                num_ways_build_design(d, &towels, &mut designs_memo).unwrap_or_default(),
            )
        })
        .collect::<Vec<(&str, u64)>>();

    let sum = answers.iter().map(|&(s, n)| n).sum::<u64>();

    //println!("{answers:?}");
    println!("{sum}");
}

fn can_build_design(design: &str, designs_memo: &mut FxHashMap<String, bool>) -> bool {
    let previous_res = designs_memo.get(design).copied();
    previous_res.map_or_else(
        || {
            let candidates = designs_memo
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
            // Special case because some original towels can actually be built multiple ways
            // from different towels
            if original_towels.contains(&design) {
                let new_towels = original_towels
                    .iter()
                    .filter(|s| **s != design)
                    .cloned()
                    .collect::<Vec<_>>();

                let res =
                    1 + num_ways_build_design_no_memo(design, &new_towels).unwrap_or_default();
                fn_memo.insert(design.to_string(), res);

                return Some(res);
            }

            if design == "" {
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

fn num_ways_build_design_no_memo(design: &str, original_towels: &[&str]) -> Option<u64> {
    if design == "" {
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
        .filter_map(|left_str| num_ways_build_design_no_memo(left_str, original_towels))
        .sum::<u64>();

    //println!("There are {branch_sum} extra ways to make {design}");

    Some(branch_sum)
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
