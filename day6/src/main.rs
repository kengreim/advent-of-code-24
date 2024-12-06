#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::*;
use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    part2();
}

fn part1() -> () {
    const PATH: &str = "day6/src/day6_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();

    let flattened = input
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<_>>();

    let num_cols = input.lines().map(|l| l.chars()).next().unwrap().count();
    let start_pos = flattened.iter().position(|l| *l == '^').unwrap();

    let mut grid = Grid::from_vec(flattened, num_cols);
    let (mut row, mut col) = (start_pos / grid.cols(), start_pos % grid.cols());

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut dir = Direction::Up;
    let mut last_pos = (row, col);
    while let Some(c) = grid.get_mut(row, col) {
        if *c == '#' {
            (row, col) = last_pos;
            dir = turn_right(&dir);
        } else {
            *c = 'X'; // Just for visual debugging, not needed
            visited.insert((row, col));
            last_pos = (row, col);
        }

        if let (Some(r), Some(c)) = step_forward(row, col, &dir) {
            (row, col) = (r, c);
        } else {
            break;
        }
    }
    println!("{}", visited.len());
}

fn part2() -> () {
    const PATH: &str = "day6/src/day6_input.txt";

    let input = fs::read_to_string(PATH).unwrap();

    let flattened = input
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<_>>();

    let num_cols = input.lines().map(|l| l.chars()).next().unwrap().count();
    let start_pos = flattened.iter().position(|l| *l == '^').unwrap();

    let grid = Grid::from_vec(flattened, num_cols);
    let (start_row, start_col) = (start_pos / grid.cols(), start_pos % grid.cols());

    // let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            match grid.get(r, c).unwrap() {
                '#' => continue,
                '^' => continue,
                _ => {
                    let mut modified_grid = grid.clone();
                    *modified_grid.get_mut(r, c).unwrap() = 'O'; // Just for visual debugging, not needed
                    if is_cyclical(
                        (start_row, start_col),
                        Direction::Up,
                        HashSet::new(),
                        modified_grid,
                    ) {
                        obstacles.insert((r, c));
                    }
                }
            }
        }
    }
    println!("{}", obstacles.len());
}

fn is_cyclical(
    current: (usize, usize),
    mut dir: Direction,
    mut visited: HashSet<(usize, usize, Direction)>,
    mut grid: Grid<char>,
) -> bool {
    let (mut row, mut col) = current;
    let (mut previous_row, mut previous_col) = (row, col);
    while let Some(c) = grid.get_mut(row, col) {
        if *c == '#' || *c == 'O' {
            (row, col) = (previous_row, previous_col);
            dir = turn_right(&dir);
        } else {
            *c = dir_to_char(c, dir.clone()); // Just for visual debugging, not needed
            if visited.contains(&(row, col, dir.clone())) {
                return true;
            }
            visited.insert((row, col, dir.clone()));
            (previous_row, previous_col) = (row, col);
        }

        if let (Some(r), Some(c)) = step_forward(row, col, &dir) {
            (row, col) = (r, c);
        } else {
            return false;
        }
    }
    false
}

fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
}

fn step_forward(row: usize, col: usize, dir: &Direction) -> (Option<usize>, Option<usize>) {
    let new_row = match (row, &dir) {
        (0, _) => None,
        (_, Direction::Up) => Some(row - 1),
        (_, Direction::Down) => Some(row + 1),
        _ => Some(row),
    };
    let new_col = match (col, &dir) {
        (0, _) => None,
        (_, Direction::Left) => Some(col - 1),
        (_, Direction::Right) => Some(col + 1),
        _ => Some(col),
    };
    (new_row, new_col)
}

fn dir_to_char(current: &char, dir: Direction) -> char {
    match dir {
        Direction::Up | Direction::Down => {
            if current == &'-' {
                '+'
            } else {
                '|'
            }
        }
        Direction::Right | Direction::Left => {
            if current == &'|' {
                '+'
            } else {
                '-'
            }
        }
    }
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
