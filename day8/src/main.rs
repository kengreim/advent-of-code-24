#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use utils::GridExt;

fn main() {
    let start = Instant::now();
    antinode_checker(true);
    println!("{:?}", start.elapsed());
}

fn antinode_checker(ignore_distance: bool) {
    const PATH: &str = "day8/src/day8_input.txt";

    let input = fs::read_to_string(PATH).unwrap();

    let grid =
        Grid::parse_from_str(&input, |l| l.chars().collect::<Vec<_>>()).expect("Invalid input");
    let mut stations = HashMap::new();
    grid.filtered_indexed_iter(|c| *c != '.')
        .for_each(|((r, c), char)| {
            stations
                .entry(char)
                .and_modify(|stations: &mut Vec<(usize, usize)>| stations.push((r, c)))
                .or_insert_with(|| vec![(r, c)]);
        });

    let mut antinodes = HashSet::new();

    for (_, instances) in stations {
        let all_pairs = instances
            .clone()
            .into_iter()
            .cartesian_product(instances)
            .filter(|&(a, b)| a != b);

        for ((station1_r, station1_c), (station2_r, station2_c)) in all_pairs {
            let station1_distance_grid = create_distance_grid((station1_r, station1_c), &grid);
            let station2_distance_grid = create_distance_grid((station2_r, station2_c), &grid);
            for ((r, c), _) in grid.indexed_iter() {
                // This "optimization", while logical, actually makes things slower
                // if positions.contains(&(r, c)) {
                //     continue;
                // }

                // Part 1 logic
                if station1_distance_grid[(r, c)] as f32 / station2_distance_grid[(r, c)] as f32
                    == 2.0
                    && !ignore_distance
                {
                    if is_double_distance(
                        (r, c),
                        (station1_r, station1_c),
                        (station2_r, station2_c),
                    ) {
                        //println!("{s}");
                        //println!("{:?}", (r, c));
                        antinodes.insert((r, c));
                    }
                }

                // Part 2 logic
                if ignore_distance
                    && ((station1_r as f32 - r as f32) / (station2_r as f32 - r as f32)
                        == (station1_c as f32 - c as f32) / (station2_c as f32 - c as f32))
                {
                    antinodes.insert((r, c));
                }
            }
        }
    }

    let mut grid2 = grid.clone();
    for (r, c) in &antinodes {
        if let Some(c) = grid2.get_mut(*r, *c) {
            *c = '#';
        }
    }
    grid2.print();
    println!("sum = {}", antinodes.len());
}

fn is_double_distance(
    point: (usize, usize),
    station1: (usize, usize),
    station2: (usize, usize),
) -> bool {
    let (r, c) = point;
    let (station1_r, station1_c) = station1;
    let (station2_r, station2_c) = station2;
    ((station1_r as f32 - r as f32) / (station2_r as f32 - r as f32) == 2.0
        && (station1_c as f32 - c as f32) / (station2_c as f32 - c as f32) == 2.0)
        || ((station1_r as f32 - r as f32) / (station2_r as f32 - r as f32) == 0.5
            && (station1_c as f32 - c as f32) / (station2_c as f32 - c as f32) == 0.5)
}

fn create_distance_grid(station_pos: (usize, usize), original_grid: &Grid<char>) -> Grid<usize> {
    let mut grid = Grid::new(original_grid.rows(), original_grid.cols());
    for ((row, col), c) in grid.indexed_iter_mut() {
        *c = station_pos.0.abs_diff(row) + station_pos.1.abs_diff(col);
    }
    grid
}
