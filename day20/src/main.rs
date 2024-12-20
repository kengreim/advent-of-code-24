#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use pathfinding::prelude::*;
use std::collections::HashMap;
use utils::GridExt;

fn main() {
    const PATH: &str = "day20/src/day20_input.txt";
    let start = std::time::Instant::now();
    part1(PATH);
    let end = start.elapsed();
    println!("{:?}", end);
}

type Cell = (usize, usize);

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();

    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<char>>()).unwrap();
    let (start, end) = find_start_and_end(&grid).unwrap();
    let (path, _) = dijkstra(&start, |&n| successors(&grid, n), |&n| n == end).unwrap();

    let mut moves_grid: Grid<Option<usize>> = Grid::new(grid.rows(), grid.cols());
    moves_grid.fill(None);
    for (steps, &cell) in path.iter().rev().enumerate() {
        moves_grid[cell] = Some(steps);
    }

    let mut cheats_map = HashMap::new();
    for cell in path {
        let current_steps = moves_grid[cell].unwrap();
        for cheat in find_possible_cheats(&grid, cell) {
            if let Some(n) = moves_grid[cheat] {
                if n < current_steps {
                    let val = current_steps - n - 2;
                    cheats_map
                        .entry(val)
                        .and_modify(|n: &mut Vec<_>| n.push((cell, cheat)))
                        .or_insert_with(|| vec![(cell, cheat)]);
                }
            }
        }
    }

    let sum = cheats_map
        .iter()
        .map(|(&n, v)| if n >= 100 { v.len() } else { 0 })
        .sum::<usize>();

    println!("{sum}");
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

fn find_possible_cheats(grid: &Grid<char>, node: Cell) -> Vec<Cell> {
    let (row, col) = node;
    let mut res = vec![];

    let offsets = vec![
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-2, 0),
        (0, 2),
        (2, 0),
        (0, -2),
    ];

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
