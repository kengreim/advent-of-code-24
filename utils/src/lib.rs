#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::cmp::Ordering;
use std::iter::repeat;

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

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

    fn parse_from_str_with_padding(
        input: &str,
        split_fn: impl Fn(&str) -> Vec<T>,
        pad_element: T,
        pad_size: usize,
    ) -> Option<Self>
    where
        Self: Sized,
        T: Clone;

    fn count_if(&self, predicate: impl Fn(&T) -> bool) -> usize;

    fn print(&self)
    where
        T: std::fmt::Display;

    fn cardinal_neighbors(&self, idx: (usize, usize)) -> Vec<((usize, usize), &T)>;

    fn cardinal_neighbors_with_direction(
        &self,
        idx: (usize, usize),
    ) -> Vec<((usize, usize), &T, Direction)>;

    fn cardinal_neighbors_with<'a>(
        &'a self,
        idx: (usize, usize),
        pred: impl Fn(&T) -> bool,
    ) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a;

    #[must_use]
    fn taxicab_distance(
        &self,
        p1: (impl TryInto<usize>, impl TryInto<usize>),
        p2: (impl TryInto<usize>, impl TryInto<usize>),
    ) -> Option<usize>;
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

    fn parse_from_str_with_padding(
        input: &str,
        split_fn: impl Fn(&str) -> Vec<T>,
        pad_element: T,
        pad_size: usize,
    ) -> Option<Self>
    where
        T: Clone,
    {
        let mut padded: Vec<T> = Vec::new();
        let lines = input.lines().collect::<Vec<_>>();
        let num_cols = split_fn(lines.first()?.trim()).len();
        let padded_num_cols = num_cols + 2 * pad_size;

        let pad_row = repeat(pad_element.clone()).take(padded_num_cols);
        padded.extend(pad_row.clone());
        for line in lines {
            for _ in 0..pad_size {
                padded.push(pad_element.clone());
            }
            padded.extend(split_fn(line));
            for _ in 0..pad_size {
                padded.push(pad_element.clone());
            }
        }
        padded.extend(pad_row);

        if padded.len() % padded_num_cols == 0 {
            Some(Self::from_vec(padded, padded_num_cols))
        } else {
            None
        }
    }

    fn count_if(&self, predicate: impl Fn(&T) -> bool) -> usize {
        self.iter().filter(|t| predicate(*t)).count()
    }

    fn print(&self)
    where
        T: std::fmt::Display,
    {
        for row in self.iter_rows() {
            let r = row.map(|s| format!("{s}")).collect::<String>();
            println!("{r}");
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

    fn cardinal_neighbors_with_direction(
        &self,
        idx: (usize, usize),
    ) -> Vec<((usize, usize), &T, Direction)> {
        let (r, c) = idx;

        let mut res = match (r.cmp(&0), c.cmp(&0)) {
            (Ordering::Greater, Ordering::Greater) => vec![
                ((r - 1, c), self.get(r - 1, c).unwrap(), Direction::Up),
                ((r, c - 1), self.get(r, c - 1).unwrap(), Direction::Left),
            ],
            (Ordering::Greater, _) => {
                vec![((r - 1, c), self.get(r - 1, c).unwrap(), Direction::Up)]
            }
            (_, Ordering::Greater) => {
                vec![((r, c - 1), self.get(r, c - 1).unwrap(), Direction::Left)]
            }
            (_, _) => vec![],
        };
        if let Some(v) = self.get(r + 1, c) {
            res.push(((r + 1, c), v, Direction::Down));
        }
        if let Some(v) = self.get(r, c + 1) {
            res.push(((r, c + 1), v, Direction::Up));
        }
        res
    }

    fn cardinal_neighbors_with<'a>(
        &'a self,
        idx: (usize, usize),
        pred: impl Fn(&'a T) -> bool,
    ) -> impl Iterator<Item = ((usize, usize), &'a T)> {
        self.cardinal_neighbors(idx)
            .into_iter()
            .filter(move |((_, _), v)| pred(v))
    }

    #[must_use]
    fn taxicab_distance(
        &self,
        p1: (impl TryInto<usize>, impl TryInto<usize>),
        p2: (impl TryInto<usize>, impl TryInto<usize>),
    ) -> Option<usize> {
        let (p1_r, p1_c): (usize, usize) = (p1.0.try_into().ok()?, p1.1.try_into().ok()?);
        let (p2_r, p2_c): (usize, usize) = (p2.0.try_into().ok()?, p2.1.try_into().ok()?);

        if p1_r < self.rows() && p2_r < self.rows() && p1_c < self.cols() && p2_c < self.cols() {
            Some(p1_r.abs_diff(p2_r) + p1_c.abs_diff(p2_c))
        } else {
            None
        }
    }
}
