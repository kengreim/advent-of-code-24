use grid::Grid;

pub fn taxicab_distance<T>(grid: Grid<T>, p1: (usize, usize), p2: (usize, usize)) -> Option<usize> {
    let rows_range = 0..grid.rows();
    let cols_range = 0..grid.cols();

    if rows_range.contains(&p1.0)
        && rows_range.contains(&p2.0)
        && cols_range.contains(&p1.1)
        && cols_range.contains(&p2.1)
    {
        Some(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
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
