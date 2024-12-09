#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;

pub trait GridExt<T> {
    fn filtered_indexed_iter<'a>(
        &'a self,
        filter_fn: impl Fn(&T) -> bool,
    ) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a;

    fn parse_from_str(input: &str, split_fn: impl Fn(&str) -> Vec<T>) -> Option<Self>
    where
        Self: Sized;

    fn count_if(&self, predicate: impl Fn(&T) -> bool) -> usize;

    fn print(&self)
    where
        T: std::fmt::Display,
        String: for<'a> FromIterator<&'a T>;
}

impl<T> GridExt<T> for Grid<T> {
    fn filtered_indexed_iter<'a>(
        &'a self,
        filter_fn: impl Fn(&T) -> bool,
    ) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a,
    {
        self.indexed_iter()
            .filter(move |((_, _), val)| filter_fn(*val))
    }

    fn parse_from_str(input: &str, split_fn: impl Fn(&str) -> Vec<T>) -> Option<Grid<T>> {
        let num_cols = split_fn(input.split_once('\n')?.0).len();
        Some(Grid::from_vec(
            input.lines().flat_map(split_fn).collect::<Vec<T>>(),
            num_cols,
        ))
    }

    fn count_if(&self, predicate: impl Fn(&T) -> bool) -> usize {
        self.iter().filter(|t| predicate(*t)).count()
    }

    fn print(&self)
    where
        T: std::fmt::Display,
        String: for<'a> FromIterator<&'a T>,
    {
        for row in self.iter_rows() {
            println!("{}", row.collect::<String>());
        }
    }
}

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
