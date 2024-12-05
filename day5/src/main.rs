#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashMap;
use std::fs;

fn main() {
    const PATH: &str = "day5/src/day5_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();

    let rules = input
        .lines()
        .filter(|l| l.contains('|'))
        .map(|l| {
            let nums = l.split('|').collect::<Vec<&str>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<_>>();

    let mut rules_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    for line in input.lines().filter(|l| l.contains('|')) {
        let nums = line.split('|').collect::<Vec<_>>();
        rules_map
            .entry(nums[0])
            .and_modify(|list| list.push((nums[0], nums[1])))
            .or_insert_with(|| vec![(nums[0], nums[1])]);
    }

    let sequences = input
        .lines()
        .filter(|l| l.contains(','))
        .collect::<Vec<_>>();

    let mut safe = vec![];
    let mut unsafe_order = vec![];
    'outer: for sequence in sequences {
        for rule in &filter_rules(&rules_map, sequence) {
            if !is_safe(rule, sequence) {
                unsafe_order.push(sequence);
                continue 'outer;
            }
        }
        safe.push(sequence);
    }

    let sum = safe
        .iter()
        .map(|s| {
            let nums = s.split(',').collect::<Vec<_>>();
            nums.get(nums.len() / 2).unwrap().parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("{sum}");

    // PART 2
    let mut fixed = vec![];
    for sequence in &unsafe_order {
        let mut sequence_vec = sequence.split(',').collect::<Vec<_>>();
        let mut safe = false;
        while !safe {
            for rule in &filter_rules(&rules_map, sequence) {
                if let (Some(a), Some(b)) = (
                    sequence_vec.iter().position(|r| *r == rule.0),
                    sequence_vec.iter().position(|r| *r == rule.1),
                ) {
                    if a > b {
                        sequence_vec.insert(b, sequence_vec[a]);
                        sequence_vec.remove(a + 1);
                    }
                }
            }

            safe = rules
                .iter()
                .map(|r| is_safe(r, sequence_vec.join(",").as_str()))
                .all(|b| b);
        }
        fixed.push(sequence_vec);
    }

    let sum2 = fixed
        .iter()
        .map(|nums| nums.get(nums.len() / 2).unwrap().parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{sum2}");
}

fn is_safe(rule: &(&str, &str), sequence: &str) -> bool {
    match (sequence.find(rule.0), sequence.find(rule.1)) {
        (Some(a), Some(b)) => a < b,
        _ => true,
    }
}

fn filter_rules<'a>(
    rules_map: &'a HashMap<&str, Vec<(&'a str, &'a str)>>,
    pair_str: &str,
) -> Vec<&'a (&'a str, &'a str)> {
    pair_str
        .split(',')
        .filter_map(|n| rules_map.get(n))
        .flatten()
        .collect::<Vec<_>>()
}
