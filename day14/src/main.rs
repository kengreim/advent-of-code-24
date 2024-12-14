#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid_util::{BoolGrid, Grid};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::sync::LazyLock;

type Pos = (usize, usize);
type Velocity = (i32, i32);

static ROBOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());
fn main() {
    const PATH: &str = "day14/src/day14_input.txt";
    let start = std::time::Instant::now();
    //part1(PATH);
    part2(PATH);
    let end = start.elapsed();
    println!("{end:?}");
}

fn part1(path: &str) {
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;
    const STEPS: usize = 100;

    let input = fs::read_to_string(path).unwrap();
    let robots = parse_robots(&input);

    let all_positions = robots
        .into_iter()
        .map(|r| robot_pos_after_steps(r, (WIDTH, HEIGHT), STEPS))
        .collect::<Vec<_>>();

    let product = make_quadrants(&all_positions, (WIDTH, HEIGHT))
        .iter()
        .flatten()
        .map(Vec::len)
        .product::<usize>();

    println!("{product}");
}

fn part2(path: &str) {
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

    let input = fs::read_to_string(path).unwrap();
    let mut robots = parse_robots(&input);

    let mut i = 0;
    loop {
        let positions = robots.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>();
        let grid = load_grid(&positions, (WIDTH, HEIGHT));

        if find_starting_positions(&positions, &grid).any(|p| score_tree(p, &grid) > 70) {
            println!("Iteration {i}");
            print_grid(&grid, '*', '.');
            break;
        }

        i += 1;
        robots = advance_robots(&robots, (WIDTH, HEIGHT));
    }
}

fn print_grid(grid: &BoolGrid, true_char: char, false_char: char) {
    let mut s = String::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            s.push(if grid.get(y, x) {
                true_char
            } else {
                false_char
            });
        }
        s.push('\n');
    }
    println!("{s}");
}

fn load_grid(positions: &HashSet<Pos>, (width, height): (usize, usize)) -> BoolGrid {
    let mut grid = BoolGrid::new(width, height, false);
    for (x, y) in positions {
        grid.set(*x, *y, true);
    }
    grid
}

fn find_starting_positions<'a>(
    positions: &'a HashSet<Pos>,
    grid: &'a BoolGrid,
) -> impl Iterator<Item = Pos> + use<'a> {
    positions
        .iter()
        .filter(|(x, y)| *y == 0 || !grid.get(*x, *y - 1))
        .copied()
}

fn score_tree((x, y): (usize, usize), grid: &BoolGrid) -> usize {
    if !grid.index_in_bounds(x, y) || !grid.get(x, y) {
        0
    } else if x > 0 {
        1 + score_tree((x - 1, y + 1), grid)
            + score_tree((x, y + 1), grid)
            + score_tree((x + 1, y + 1), grid)
    } else {
        1 + score_tree((x, y + 1), grid) + score_tree((x + 1, y + 1), grid)
    }
}

fn make_quadrants(positions: &[Pos], (width, height): (usize, usize)) -> Vec<Vec<Vec<Pos>>> {
    let mut res: Vec<Vec<Vec<Pos>>> = vec![vec![vec![], vec![]], vec![vec![], vec![]]];
    let x_mid = width / 2;
    let y_mid = height / 2;

    for (x, y) in positions {
        if *x == x_mid || *y == y_mid {
            continue;
        }

        let x_quad = usize::from(*x >= x_mid);
        let y_quad = usize::from(*y >= y_mid);
        res[x_quad][y_quad].push((*x, *y));
    }

    res
}

#[allow(dead_code)]
fn robot_walk(
    (pos, v): (Pos, Velocity),
    (width, height): (usize, usize),
    steps: usize,
) -> Vec<Pos> {
    (0..=steps)
        .map(|i| robot_pos_after_steps((pos, v), (width, height), i))
        .collect()
}

const fn robot_pos_after_steps(
    (pos, v): (Pos, Velocity),
    (width, height): (usize, usize),
    steps: usize,
) -> Pos {
    let (start_x, start_y) = pos;
    let (vx, vy) = v;
    (
        (start_x as i32 + steps as i32 * vx).rem_euclid(width as i32) as usize,
        (start_y as i32 + steps as i32 * vy).rem_euclid(height as i32) as usize,
    )
}

const fn advance_robot(
    (pos, v): (Pos, Velocity),
    (width, height): (usize, usize),
) -> (Pos, Velocity) {
    let (start_x, start_y) = pos;
    let (vx, vy) = v;
    (
        (
            (start_x as i32 + vx).rem_euclid(width as i32) as usize,
            (start_y as i32 + vy).rem_euclid(height as i32) as usize,
        ),
        v,
    )
}

fn advance_robots(
    robots: &[(Pos, Velocity)],
    (width, height): (usize, usize),
) -> Vec<(Pos, Velocity)> {
    robots
        .iter()
        .map(|r| advance_robot(*r, (width, height)))
        .collect()
}

fn parse_robots(input: &str) -> Vec<(Pos, Velocity)> {
    let res = input
        .lines()
        .filter_map(|line| {
            let caps = ROBOT_RE.captures(line)?;
            Some((
                (
                    caps[1].to_string().parse::<usize>().ok()?,
                    caps[2].to_string().parse::<usize>().ok()?,
                ),
                (
                    caps[3].to_string().parse::<i32>().ok()?,
                    caps[4].to_string().parse::<i32>().ok()?,
                ),
            ))
        })
        .collect::<Vec<_>>();
    res
}
