#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use log::warn;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::iter::repeat;
use std::time::Instant;
use utils::GridExt;

fn main() {
    const PATH: &str = "day12/src/day12_example.txt";
    let start = Instant::now();
    part2(PATH);
    println!("{:?}", start.elapsed());
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<_>>()).unwrap();

    let mut visited = HashSet::new();
    let mut sum = 0;

    for ((r1, c1), char) in grid.indexed_iter() {
        if visited.contains(&(r1, c1)) {
            continue;
        }

        let mut current_region: Vec<(usize, usize)> = vec![(r1, c1)];
        let mut current_region_perim = 0;

        visited.insert((r1, c1));
        let mut queue = VecDeque::from(vec![(r1, c1)]);
        while !queue.is_empty() {
            let (r2, c2) = queue.pop_front().unwrap();

            let mut cell_perim = 4;
            for (idx, _) in grid.cardinal_neighbors_with((r2, c2), |val| *val == *char) {
                if !visited.contains(&idx) {
                    queue.push_back(idx);
                    visited.insert(idx);
                    current_region.push(idx);
                }
                cell_perim -= 1;
            }
            current_region_perim += cell_perim;
        }
        sum += current_region.len() * current_region_perim;
    }
    println!("sum: {sum}");
}

fn get_regions(grid: &Grid<char>) -> Vec<(char, Vec<(usize, usize)>)> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for ((r1, c1), char) in grid.indexed_iter() {
        if *char == '.' || visited.contains(&(r1, c1)) {
            continue;
        }

        let mut current_region: Vec<(usize, usize)> = vec![(r1, c1)];
        let mut current_region_perim = 0;

        visited.insert((r1, c1));
        let mut queue = VecDeque::from(vec![(r1, c1)]);
        while !queue.is_empty() {
            let (r2, c2) = queue.pop_front().unwrap();

            let mut cell_perim = 4;
            for (idx, _) in grid.cardinal_neighbors_with((r2, c2), |val| *val == *char) {
                if !visited.contains(&idx) {
                    queue.push_back(idx);
                    visited.insert(idx);
                    current_region.push(idx);
                }
                cell_perim -= 1;
            }
            current_region_perim += cell_perim;
        }
        regions.push((*char, current_region));
    }
    regions
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (padded_vec, num_cols) = pad_grid_from_str(&input, '.', 1).unwrap();

    let grid = Grid::from_vec(padded_vec, num_cols);
    let regions = get_regions(&grid);
    for (ch, coords) in regions {
        let sum = coords
            .iter()
            .map(|(r, c)| corner_val(&grid, (*r, *c)))
            .sum::<i32>();
        println!("{sum}");
    }
}

fn pad_grid_from_str(
    original: &str,
    pad_char: char,
    pad_size: usize,
) -> Option<(Vec<char>, usize)> {
    let mut padded = Vec::new();
    let lines = original.lines().collect::<Vec<_>>();
    let num_cols = lines.get(0)?.trim().chars().count();

    let pad_row = repeat(pad_char)
        .take(num_cols + 2 * pad_size)
        .collect::<Vec<_>>();
    padded.extend(pad_row.clone());
    for line in lines {
        for _ in 0..pad_size {
            padded.push(pad_char);
        }
        padded.extend(line.trim().chars());
        for _ in 0..pad_size {
            padded.push(pad_char);
        }
    }
    padded.extend(pad_row);

    Some((padded, num_cols + 2 * pad_size))
}

fn corner_val(grid: &Grid<char>, pos: (usize, usize)) -> i32 {
    let (r, c) = pos;
    let ch = grid[(r, c)];
    let (left, up, right, down) = (
        grid[(r, c - 1)] == ch,
        grid[(r - 1, c)] == ch,
        grid[(r, c + 1)] == ch,
        grid[(r + 1, c)] == ch,
    );

    // let v = match (left, up, right, down) {
    //     (false, false, false, false) => 4,
    //     (true, false, false, false) => 2,
    //     (true, true, false, false) => 1,
    //     (true, true, true, false) => 0,
    //     (true, true, true, true) => 0,
    //     (false, true, false, false) => 2,
    //     (false, true, true, false) => 1,
    //     (false, true, true, true) => 0,
    //     (false, false, true, false) => 2,
    //     (false, false, true, true) => 1,
    //     (true, false, true, false) => 0,
    //     (true, false, true, true) => 1,
    //     (false, true, false, true) => 0,
    //     (false, false, false, true) => 2,
    //     (true, false, false, true) => 1,
    //     (true, true, false, true) => 0,
    // };
    let v = match (left, up, right, down) {
        (true, false, false, false) => 3,
        (false, true, false, false) => 3,
        (false, false, true, false) => 3,
        (false, false, false, true) => 3,
        (false, false, false, false) => 4,
        (true, true, false, false) => 1,
        (false, true, true, false) => 1,
        (false, false, true, true) => 1,
        (true, false, false, true) => 1,
        _ => 0,
    };
    println!("{v} {:?}", pos);
    //println!("{left} {up} {right} {down}");
    v
}
