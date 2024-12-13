#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use num::Integer;
use regex::Regex;
use std::sync::LazyLock;
use std::time::Instant;
use std::{cmp, fs};

static BUTTON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Button \w: \w\+(\d+), \w\+(\d+)").unwrap());

static PRIZE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Prize: \w=(\d+), \w=(\d+)").unwrap());

fn main() {
    const PATH: &str = "day13/src/day13_input.txt";

    let start = Instant::now();
    part1(PATH);
    println!("Part 1 elapsed {:?}", start.elapsed());

    let start = Instant::now();
    part2(PATH);
    println!("Part 2 elapsed {:?}", start.elapsed());
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let machines = parse_machines(&input, (0, 0)).unwrap();

    let sum = machines
        .iter()
        .filter_map(|(a, b, prize)| {
            find_machine_solutions(a, b, prize)
                .into_iter()
                .map(score)
                .min()
        })
        .sum::<i64>();
    println!("Part 1: {sum}");
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let machines = parse_machines(&input, (10_000_000_000_000, 10_000_000_000_000)).unwrap();

    let sum = machines
        .iter()
        .filter_map(|(a, b, prize)| find_machine_solution_2(a, b, prize))
        .map(score)
        .sum::<i64>();
    println!("Part 2: {sum}");
}

fn parse_machines(
    input: &str,
    prize_offset: (i64, i64),
) -> Option<Vec<((i64, i64), (i64, i64), (i64, i64))>> {
    let mut machines = Vec::new();
    for lines in input.lines().collect::<Vec<&str>>().chunks(4) {
        let a_captures = BUTTON_RE.captures(lines[0])?;
        let b_captures = BUTTON_RE.captures(lines[1])?;
        let prize_captures = PRIZE_RE.captures(lines[2])?;
        machines.push((
            (
                a_captures[1].to_string().parse::<i64>().ok()?,
                a_captures[2].to_string().parse::<i64>().ok()?,
            ),
            (
                b_captures[1].to_string().parse::<i64>().ok()?,
                b_captures[2].to_string().parse::<i64>().ok()?,
            ),
            (
                prize_captures[1].to_string().parse::<i64>().ok()? + prize_offset.0,
                prize_captures[2].to_string().parse::<i64>().ok()? + prize_offset.1,
            ),
        ));
    }
    Some(machines)
}

fn find_machine_solutions(a: &(i64, i64), b: &(i64, i64), prize: &(i64, i64)) -> Vec<(i64, i64)> {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (prize_x, prize_y) = prize;
    let x_gcd = i64::extended_gcd(ax, bx);
    let y_gcd = i64::extended_gcd(ay, by);

    if prize_x % x_gcd.gcd != 0 || prize_y % y_gcd.gcd != 0 {
        return vec![];
    }

    let mut solutions = vec![];

    let max_a_press = cmp::max(prize_x / ax, prize_y / ay);
    for a_press in 0..max_a_press {
        let b_press1 = (*prize_x as f64 - (*ax as f64 * a_press as f64)) / *bx as f64;
        let b_press2 = (*prize_y as f64 - (*ay as f64 * a_press as f64)) / *by as f64;
        if b_press1.fract() == 0.0 && b_press2.fract() == 0.0 && b_press1 == b_press2 {
            solutions.push((a_press, b_press1 as i64));
        }
    }

    solutions
}

fn find_machine_solution_2(
    a: &(i64, i64),
    b: &(i64, i64),
    prize: &(i64, i64),
) -> Option<(i64, i64)> {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (prize_x, prize_y) = prize;

    let numerator = (bx * prize_y) - (by * prize_x);
    let denominator = (ay * bx) - (by * ax);

    let a_press = numerator as f64 / denominator as f64;
    if a_press.fract() == 0.0 {
        let b_press = (prize_x - (ax * a_press as i64)) / bx;
        Some((a_press as i64, b_press))
    } else {
        None
    }
}

const fn score(solution: (i64, i64)) -> i64 {
    solution.0 * 3 + solution.1
}
