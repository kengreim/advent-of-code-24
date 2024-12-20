#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use pathfinding::prelude::*;
use std::collections::{HashMap, HashSet};
use utils::GridExt;

fn main() {
    const PATH: &str = "day20/src/day20_input.txt";
    let start = std::time::Instant::now();
    part1(PATH);
    part2(PATH);
    let end = start.elapsed();
    println!("{:?}", end);
}

type Cell = (usize, usize);

fn part1(path: &str) {
    let cheats_map = solve_cheats(path, 2, 2, 100);
    let sum = cheats_map.values().map(|v| v.len()).sum::<usize>();
    println!("{sum}");
}

fn part2(path: &str) {
    let cheats_map = solve_cheats(path, 2, 20, 100);
    let sum = cheats_map.values().map(|v| v.len()).sum::<usize>();
    println!("{sum}");
}

fn solve_cheats(
    path: &str,
    min_distance: usize,
    max_distance: usize,
    min_cheat: usize,
) -> HashMap<usize, Vec<(Cell, Cell)>> {
    let input = std::fs::read_to_string(path).unwrap();

    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<char>>()).unwrap();
    let (start, end) = find_start_and_end(&grid).unwrap();
    let (path, _) = dijkstra(&start, |&n| successors(&grid, n), |&n| n == end).unwrap();
    let distance_grid = make_distance_grid(&grid, &path);

    let mut cheats_map = HashMap::new();
    for cell in path {
        let current_steps = distance_grid[cell].unwrap();
        for cheat in find_possible_cheats(&grid, cell, min_distance, max_distance) {
            if let Some(n) = distance_grid[cheat] {
                let manhattan = cell.0.abs_diff(cheat.0) + cell.1.abs_diff(cheat.1);
                let cheat_val = current_steps - n - manhattan;
                if n < current_steps && cheat_val >= min_cheat {
                    cheats_map
                        .entry(cheat_val)
                        .and_modify(|n: &mut Vec<_>| {
                            n.push((cell, cheat));
                        })
                        .or_insert_with(|| vec![(cell, cheat)]);
                }
            }
        }
    }

    cheats_map
}

fn print_grid(grid: &Grid<Option<usize>>) {
    let mut s = String::new();
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let cell = if let Some(n) = grid[(row, col)] {
                &format!("[{n:02}]")
            } else {
                "[..]"
            };

            s.push_str(cell);
        }
        s.push('\n');
    }
    println!("{s}");
}

fn make_distance_grid(grid: &Grid<char>, path: &[Cell]) -> Grid<Option<usize>> {
    let mut distance_grid: Grid<Option<usize>> = Grid::new(grid.rows(), grid.cols());
    distance_grid.fill(None);
    for (steps, &cell) in path.iter().rev().enumerate() {
        distance_grid[cell] = Some(steps);
    }

    distance_grid
}

fn find_start_and_end(grid: &Grid<char>) -> Option<(Cell, Cell)> {
    let mut end = None;
    let mut start = None;
    for ((r, c), &val) in grid.indexed_iter() {
        if val == 'E' {
            end = Some((r, c));
        } else if val == 'S' {
            start = Some((r, c));
        }
    }
    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

fn find_possible_cheats(
    grid: &Grid<char>,
    node: Cell,
    min_distance: usize,
    max_distance: usize,
) -> Vec<Cell> {
    let (row, col) = node;
    let mut res = vec![];
    let offsets = (min_distance..=max_distance).flat_map(|n| offsets(n));

    for (r_off, c_off) in offsets {
        let (r, c) = (row as i32 + r_off, col as i32 + c_off);
        if r >= 0 && c >= 0 {
            if let Some(&ch) = grid.get(r, c) {
                if ch != '#' {
                    res.push((r as usize, c as usize));
                }
            }
        }
    }

    res
}

fn offsets(distance: usize) -> Vec<(i32, i32)> {
    let mut offsets = vec![];

    for i in 0..=distance {
        let j = distance - i;
        offsets.push((i as i32, j as i32));
        if i != 0 {
            offsets.push((-(i as i32), j as i32));
        }
        if j != 0 {
            offsets.push((i as i32, -(j as i32)));
        }
        if i != 0 && j != 0 {
            offsets.push((-(i as i32), -(j as i32)));
        }
    }

    offsets
}

fn successors(grid: &Grid<char>, node: Cell) -> Vec<(Cell, usize)> {
    let (row, col) = node;
    let mut res = vec![];

    if row > 0 {
        if let Some(&c) = grid.get(row - 1, col) {
            if c != '#' {
                res.push(((row - 1, col), 1));
            }
        }
    }

    if col > 0 {
        if let Some(&c) = grid.get(row, col - 1) {
            if c != '#' {
                res.push(((row, col - 1), 1));
            }
        }
    }

    if let Some(&c) = grid.get(row, col + 1) {
        if c != '#' {
            res.push(((row, col + 1), 1));
        }
    }

    if let Some(&c) = grid.get(row + 1, col) {
        if c != '#' {
            res.push(((row + 1, col), 1));
        }
    }

    res
}
