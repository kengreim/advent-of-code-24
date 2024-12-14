#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use regex::Regex;
use std::fs;
use std::sync::LazyLock;

type Pos = (i32, i32);
type Velocity = (i32, i32);

static ROBOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());
fn main() {
    const PATH: &str = "day14/src/day14_input.txt";
    let start = std::time::Instant::now();
    part1(PATH);
    let end = start.elapsed();
    println!("{:?}", end);
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let robots = parse_robots(&input).unwrap();
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;
    const STEPS: usize = 100;

    let all_positions = robots
        .into_iter()
        .map(|r| robot_pos_after_steps(r, (WIDTH, HEIGHT), STEPS))
        .collect::<Vec<_>>();

    let product = make_quadrants(&all_positions, (WIDTH, HEIGHT))
        .iter()
        .flatten()
        .map(|q| q.len())
        .product::<usize>();

    println!("{product}");
}

fn make_quadrants(positions: &[Pos], (width, height): (usize, usize)) -> Vec<Vec<Vec<Pos>>> {
    let mut res: Vec<Vec<Vec<(i32, i32)>>> = vec![vec![vec![], vec![]], vec![vec![], vec![]]];
    let x_mid = width / 2;
    let y_mid = height / 2;

    for (x, y) in positions {
        if *x == x_mid as i32 || *y == y_mid as i32 {
            continue;
        }

        let x_quad = if *x < x_mid as i32 { 0 } else { 1 };
        let y_quad = if *y < y_mid as i32 { 0 } else { 1 };
        res[x_quad][y_quad].push((*x, *y))
    }

    res
}

fn robot_walk(
    (pos, v): (Pos, Velocity),
    (width, height): (usize, usize),
    steps: usize,
) -> Vec<Pos> {
    (0..=steps)
        .map(|i| robot_pos_after_steps((pos, v), (width, height), i))
        .collect()
}

fn robot_pos_after_steps(
    (pos, v): (Pos, Velocity),
    (width, height): (usize, usize),
    steps: usize,
) -> Pos {
    let (start_x, start_y) = pos;
    let (vx, vy) = v;
    (
        (start_x + steps as i32 * vx).rem_euclid(width as i32),
        (start_y + steps as i32 * vy).rem_euclid(height as i32),
    )
}

fn parse_robots(input: &str) -> Option<Vec<(Pos, Velocity)>> {
    let res = input
        .lines()
        .into_iter()
        .filter_map(|line| {
            let caps = ROBOT_RE.captures(line)?;
            Some((
                (
                    caps[1].to_string().parse::<i32>().ok()?,
                    caps[2].to_string().parse::<i32>().ok()?,
                ),
                (
                    caps[3].to_string().parse::<i32>().ok()?,
                    caps[4].to_string().parse::<i32>().ok()?,
                ),
            ))
        })
        .collect::<Vec<_>>();
    Some(res)
}
