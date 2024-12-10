#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use utils::GridExt;

fn main() {
    const PATH: &str = "day10/src/day10_input.txt";
    let start = Instant::now();
    run(PATH);
    println!("Duration {:?}", start.elapsed());
}

fn run(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| {
        l.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    })
    .unwrap();

    let trailheads = grid
        .filtered_indexed_iter(|n| *n == 0)
        .map(idx_only)
        .collect::<Vec<_>>();

    let sum: usize = trailheads
        .iter()
        .map(|idx| paths_to_peak_p1(&grid, HashSet::from([*idx]), 1).len())
        .sum();

    let sum2: usize = trailheads
        .iter()
        .map(|idx| paths_to_peak_p2(&grid, vec![*idx], 1).len())
        .sum();

    println!("Part 1: {sum:?}");
    println!("Part 2: {sum2:?}");
}

fn paths_to_peak_p1(
    grid: &Grid<u32>,
    initial_points: HashSet<(usize, usize)>,
    next_val: u32,
) -> HashSet<(usize, usize)> {
    if next_val > 9 || initial_points.len() == 0 {
        initial_points
    } else {
        let new_points = initial_points
            .iter()
            .flat_map(|idx| {
                grid.cardinal_neighbors_with(*idx, |v| *v == next_val)
                    .map(idx_only)
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<(usize, usize)>>();
        paths_to_peak_p1(grid, new_points, next_val + 1)
    }
}

fn paths_to_peak_p2(
    grid: &Grid<u32>,
    initial_points: Vec<(usize, usize)>,
    next_val: u32,
) -> Vec<(usize, usize)> {
    if next_val > 9 || initial_points.len() == 0 {
        initial_points
    } else {
        let new_points = initial_points
            .iter()
            .flat_map(|idx| {
                grid.cardinal_neighbors_with(*idx, |v| *v == next_val)
                    .map(idx_only)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        paths_to_peak_p2(grid, new_points, next_val + 1)
    }
}

fn idx_only<T>(val: ((usize, usize), &T)) -> (usize, usize) {
    (val.0 .0, val.0 .1)
}
