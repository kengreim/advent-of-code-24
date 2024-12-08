#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;

#[must_use]
pub fn taxicab_distance<T>(
    grid: &Grid<T>,
    p1: (impl TryInto<usize>, impl TryInto<usize>),
    p2: (impl TryInto<usize>, impl TryInto<usize>),
) -> Option<usize> {
    let (p1_r, p1_c): (usize, usize) = (p1.0.try_into().ok()?, p1.1.try_into().ok()?);
    let (p2_r, p2_c): (usize, usize) = (p2.0.try_into().ok()?, p2.1.try_into().ok()?);

    if p1_r < grid.rows() && p2_r < grid.rows() && p1_c < grid.cols() && p2_c < grid.cols() {
        Some(p1_r.abs_diff(p2_r) + p1_c.abs_diff(p2_c))
    } else {
        None
    }
}

pub fn parse_grid<T>(input: &str, split_fn: impl Fn(&str) -> Vec<T>) -> Option<Grid<T>> {
    let num_cols = split_fn(input.split_once('\n')?.0).len();
    Some(Grid::from_vec(
        input.lines().flat_map(split_fn).collect::<Vec<T>>(),
        num_cols,
    ))
}
