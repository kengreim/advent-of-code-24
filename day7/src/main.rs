#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part2_recursive();
    println!("{:?}", start.elapsed());
}

fn part1() {
    const PATH: &str = "day7/src/day7_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let (total, seq) = parse_line(line);
        let initial_possibilities = calc_possibilities_part1(&[seq[0]], seq[1]);

        let all_possibilities = seq.iter().skip(2).fold(initial_possibilities, |acc, n| {
            calc_possibilities_part1(&acc, *n)
        });

        if all_possibilities.contains(&total) {
            sum += total;
        }
    }
    println!("{sum}");
}

fn part2() {
    const PATH: &str = "day7/src/day7_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let (total, seq) = parse_line(line);
        let initial_possibilities = calc_possibilities_part2(&[seq[0]], seq[1]);

        let all_possibilities = seq.iter().skip(2).fold(initial_possibilities, |acc, n| {
            calc_possibilities_part2(&acc, *n)
        });

        if all_possibilities.contains(&total) {
            sum += total;
        }
    }
    println!("{sum}");
}

fn part2_recursive() {
    const PATH: &str = "day7/src/day7_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let (total, seq) = parse_line(line);
        if can_be_valid(total, &seq, seq.len() - 1) {
            sum += total;
        }
    }
    println!("{sum}");
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    if let Some((total, nums)) = line.split_once(':') {
        (
            total.parse::<i64>().unwrap(),
            nums.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    } else {
        panic!()
    }
}

fn calc_possibilities_part1(possible_totals: &[i64], next_num: i64) -> Vec<i64> {
    possible_totals
        .iter()
        .flat_map(|t| vec![t + next_num, t * next_num])
        .collect()
}

fn calc_possibilities_part2(possible_totals: &[i64], next_num: i64) -> Vec<i64> {
    possible_totals
        .iter()
        .flat_map(|t| vec![t + next_num, t * next_num, concat_nums(*t, next_num)])
        .collect()
}

fn concat_nums(a: i64, b: i64) -> i64 {
    format!("{a}{b}").parse().unwrap()
}

fn can_be_valid(total: i64, seq: &[i64], idx: usize) -> bool {
    if idx == 1 {
        total.to_string() == format!("{}{}", seq[0], seq[1])
            || total == seq[0] + seq[1]
            || total == seq[0] * seq[1]
    } else {
        let mut possibilties = vec![];

        if let Some(s) = unconcat(&total.to_string(), &seq[idx].to_string()) {
            possibilties.push((&seq[0..idx], idx - 1, s.parse::<i64>().unwrap()));
        }

        if total % seq[idx] == 0 {
            possibilties.push((&seq[0..idx], idx - 1, total / seq[idx]))
        }

        if total - seq[idx] > 0 {
            possibilties.push((&seq[0..idx], idx - 1, total - seq[idx]));
        }

        possibilties
            .iter()
            .map(|(seq, idx, total)| can_be_valid(*total, seq, *idx))
            .any(|b| b)
    }
}

fn unconcat(full_str: &str, end_str: &str) -> Option<String> {
    if full_str.ends_with(end_str) && full_str != end_str {
        Some(full_str[0..full_str.len() - end_str.len()].to_string())
    } else {
        None
    }
}
