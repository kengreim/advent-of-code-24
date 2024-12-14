#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use regex::Regex;
use std::fs;
use std::sync::LazyLock;

type Pos = (i32, i32);
type Velocity = (i32, i32);

static ROBOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());
fn main() {
    const PATH: &str = "day14/src/day14_example.txt";
    part1(PATH);
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let robots = parse_robots(&input).unwrap();
    let all_positions = robots
        .into_iter()
        .map(|r| robot_pos_after_steps(r, (11, 7), 100))
        .collect::<Vec<_>>();
    println!("{:?}", all_positions);
}

fn robot_walk((pos, v): (Pos, Velocity), (max_x, max_y): (i32, i32), steps: usize) -> Vec<Pos> {
    (0..=steps)
        .map(|i| robot_pos_after_steps((pos, v), (max_x, max_y), i))
        .collect()
}

fn robot_pos_after_steps(
    (pos, v): (Pos, Velocity),
    (max_x, max_y): (i32, i32),
    steps: usize,
) -> Pos {
    let (start_x, start_y) = pos;
    let (vx, vy) = v;
    (
        (start_x + steps as i32 * vx).rem_euclid(max_x),
        (start_y + steps as i32 * vy).rem_euclid(max_y),
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
