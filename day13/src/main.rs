use num::Integer;
use regex::Regex;
use std::collections::HashSet;
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
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let machines = parse_machines(&input).unwrap();

    let sum = machines
        .iter()
        .flat_map(|(a, b, prize)| {
            find_machine_solutions(*a, *b, *prize)
                .iter()
                .map(score)
                .max()
        })
        .sum::<i32>();
    println!("Part 1: {}", sum);
}

fn parse_machines(input: &str) -> Option<Vec<((i32, i32), (i32, i32), (i32, i32))>> {
    let mut machines = Vec::new();
    for lines in input.lines().collect::<Vec<&str>>().chunks(4) {
        let a_captures = BUTTON_RE.captures(lines[0])?;
        let b_captures = BUTTON_RE.captures(lines[1])?;
        let prize_captures = PRIZE_RE.captures(lines[2])?;
        machines.push((
            (
                a_captures[1].to_string().parse::<i32>().ok()?,
                a_captures[2].to_string().parse::<i32>().ok()?,
            ),
            (
                b_captures[1].to_string().parse::<i32>().ok()?,
                b_captures[2].to_string().parse::<i32>().ok()?,
            ),
            (
                prize_captures[1].to_string().parse::<i32>().ok()?,
                prize_captures[2].to_string().parse::<i32>().ok()?,
            ),
        ));
    }
    Some(machines)
}

fn find_machine_solutions(a: (i32, i32), b: (i32, i32), prize: (i32, i32)) -> Vec<(i32, i32)> {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (prize_x, prize_y) = prize;
    let x_gcd = i32::extended_gcd(&ax, &bx);
    let y_gcd = i32::extended_gcd(&ay, &by);

    if prize_x % x_gcd.gcd != 0 || prize_y % y_gcd.gcd != 0 {
        return vec![];
    }

    let mut solutions = vec![];

    let max_a_press = cmp::max(prize_x / ax, prize_y / ay);
    for a_press in 0..max_a_press {
        let b_press1 = (prize_x as f64 - (ax as f64 * a_press as f64)) / bx as f64;
        let b_press2 = (prize_y as f64 - (ay as f64 * a_press as f64)) / by as f64;
        if b_press1.fract() == 0.0 && b_press2.fract() == 0.0 && b_press1 == b_press2 {
            solutions.push((a_press, b_press1 as i32));
        }
    }
    if solutions.len() > 1 {
        println!("multiple");
    }
    solutions
}

fn score(solution: &(i32, i32)) -> i32 {
    solution.0 * 3 + solution.1
}
