#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::cmp::Ordering;

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

    fn cardinal_neighbors(&self, idx: (usize, usize)) -> Vec<((usize, usize), &T)>;
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

    fn parse_from_str(input: &str, split_fn: impl Fn(&str) -> Vec<T>) -> Option<Self> {
        let num_cols = split_fn(input.split_once('\n')?.0).len();
        Some(Self::from_vec(
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

    fn cardinal_neighbors(&self, idx: (usize, usize)) -> Vec<((usize, usize), &T)> {
        let (r, c) = idx;

        let mut res = match (r.cmp(&0), c.cmp(&0)) {
            (Ordering::Greater, Ordering::Greater) => vec![
                ((r - 1, c), self.get(r - 1, c).unwrap()),
                ((r, c - 1), self.get(r, c - 1).unwrap()),
            ],
            (Ordering::Greater, _) => vec![((r - 1, c), self.get(r - 1, c).unwrap())],
            (_, Ordering::Greater) => vec![((r, c - 1), self.get(r, c - 1).unwrap())],
            (_, _) => vec![],
        };
        if let Some(v) = self.get(r + 1, c) {
            res.push(((r + 1, c), v));
        }
        if let Some(v) = self.get(r, c + 1) {
            res.push(((r, c + 1), v));
        }
        res
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
