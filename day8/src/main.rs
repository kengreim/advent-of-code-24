use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    antinode_checker(true)
}

fn antinode_checker(ignore_distance: bool) {
    const PATH: &str = "day8/src/day8_input.txt";

    let input = fs::read_to_string(PATH).unwrap();
    let mut stations = input
        .lines()
        .flat_map(|l| l.chars())
        .filter(|c| *c != '.')
        .collect::<Vec<_>>();
    stations.sort();
    stations.dedup();

    let num_cols = input.lines().map(|l| l.chars()).next().unwrap().count();
    let grid = Grid::from_vec(
        input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>(),
        num_cols,
    );

    let mut positions = HashSet::new();

    for s in stations {
        let instances = grid
            .indexed_iter()
            .filter(|((_, _), c)| **c == s)
            .map(|((row, col), _)| (row, col))
            .collect::<Vec<_>>();

        let all_pairs = instances
            .clone()
            .into_iter()
            .cartesian_product(instances.clone())
            .filter(|&(a, b)| a != b);

        for (station1, station2) in all_pairs {
            let station1_distance_grid = create_distance_grid(station1, &grid);
            let station2_distance_grid = create_distance_grid(station2, &grid);
            for ((r, c), _) in grid.indexed_iter() {
                // Part 1 logic
                if station1_distance_grid[(r, c)] as f32 / station2_distance_grid[(r, c)] as f32
                    == 2.0
                    && !ignore_distance
                {
                    let (closer_row, closer_col, farther_row, farther_col) =
                        if station1_distance_grid[(r, c)] < station2_distance_grid[(r, c)] {
                            (station1.0, station1.1, station2.0, station2.1)
                        } else {
                            (station2.0, station2.1, station1.0, station1.1)
                        };

                    if (farther_row as f32 - r as f32) / (closer_row as f32 - r as f32) == 2.0
                        && (farther_col as f32 - c as f32) / (closer_col as f32 - c as f32) == 2.0
                    {
                        //println!("{s}");
                        //println!("{:?}", (r, c));
                        positions.insert((r, c));
                    }
                }

                // Part 2 logic
                if ignore_distance {
                    if (station1.0 as f32 - r as f32) / (station2.0 as f32 - r as f32)
                        == (station1.1 as f32 - c as f32) / (station2.1 as f32 - c as f32)
                    {
                        positions.insert((r, c));
                    }
                }
            }
        }
    }
    println!("sum = {}", positions.iter().count());
    let mut grid2 = grid.clone();
    for (r, c) in positions {
        if let Some(c) = grid2.get_mut(r, c) {
            *c = '#';
        }
    }
    print_grid(&grid2);
}

fn create_distance_grid(station_pos: (usize, usize), original_grid: &Grid<char>) -> Grid<usize> {
    let mut grid = Grid::new(original_grid.rows(), original_grid.cols());
    for ((row, col), c) in grid.indexed_iter_mut() {
        *c = station_pos.0.abs_diff(row) + station_pos.1.abs_diff(col);
    }
    grid
}

fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{:?}", row.collect::<String>());
    }
}
