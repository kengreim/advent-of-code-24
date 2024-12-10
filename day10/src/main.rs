#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use utils::GridExt;

fn main() {
    const PATH: &str = "day10/src/day10_input.txt";
    let start = Instant::now();
    part1(PATH);
    println!("Duration {:?}", start.elapsed());
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    })
    .unwrap();

    let trailheads = grid
        .filtered_indexed_iter(|n| *n == 0)
        .map(|((r, c), _)| ((r, c), 0))
        .collect::<HashMap<(usize, usize), i32>>();

    let sum = trailheads
        .keys()
        .map(|idx| {
            let mut set = HashSet::new();
            set.insert(*idx);
            paths_to_peak_p1(&grid, set, 1).iter().len()
        })
        .sum::<usize>();

    let sum2 = trailheads
        .keys()
        .map(|idx| paths_to_peak_p2(&grid, vec![*idx], 1).iter().len())
        .sum::<usize>();

    println!("Part 1: {sum:?}");
    println!("Part 2: {sum2:?}");
}

fn paths_to_peak_p1(
    grid: &Grid<u32>,
    initial_points: HashSet<(usize, usize)>,
    next_val: u32,
) -> HashSet<(usize, usize)> {
    if next_val > 9 {
        initial_points
    } else {
        let new_points = initial_points
            .iter()
            .flat_map(|idx| cardinal_neighbors_with(grid, *idx, |v| v == next_val))
            .collect::<HashSet<(usize, usize)>>();
        paths_to_peak_p1(grid, new_points, next_val + 1)
    }
}

fn paths_to_peak_p2(
    grid: &Grid<u32>,
    initial_points: Vec<(usize, usize)>,
    next_val: u32,
) -> Vec<(usize, usize)> {
    if next_val > 9 {
        initial_points
    } else {
        let new_points = initial_points
            .iter()
            .flat_map(|idx| cardinal_neighbors_with(grid, *idx, |v| v == next_val))
            .collect::<Vec<_>>();
        paths_to_peak_p2(grid, new_points, next_val + 1)
    }
}

fn cardinal_neighbors_with(
    grid: &Grid<u32>,
    idx: (usize, usize),
    pred: impl Fn(u32) -> bool,
) -> Vec<(usize, usize)> {
    grid.cardinal_neighbors(idx)
        .iter()
        .filter_map(
            |((r, c), val)| {
                if pred(**val) {
                    Some((*r, *c))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>()
}
