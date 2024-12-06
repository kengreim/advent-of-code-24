#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let (initial, visited) = part1();

    let start = Instant::now();
    part2(initial, &visited);
    println!("Finished part 2 after {:?}", start.elapsed());
}

fn part1() -> ((usize, usize), Grid<char>) {
    const PATH: &str = "day6/src/day6_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();

    let flattened = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let num_cols = input.lines().map(|l| l.chars()).next().unwrap().count();
    let start_pos = flattened.iter().position(|l| *l == '^').unwrap();

    let mut grid = Grid::from_vec(flattened, num_cols);
    let (start_row, start_col) = (start_pos / grid.cols(), start_pos % grid.cols());
    let (mut row, mut col) = (start_row, start_col);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut dir = Direction::Up;
    let mut last_pos = (row, col);
    while let Some(c) = grid.get_mut(row, col) {
        if *c == '#' {
            (row, col) = last_pos;
            dir = turn_right(dir);
        } else {
            *c = 'X'; // Just for visual debugging, not needed
            visited.insert((row, col));
            last_pos = (row, col);
        }

        if let (Some(r), Some(c)) = step_forward(row, col, dir) {
            (row, col) = (r, c);
        } else {
            break;
        }
    }
    println!("{}", visited.len());
    ((start_row, start_col), grid)
}

fn part2(initial_pos: (usize, usize), grid: &Grid<char>) {
    let (start_row, start_col) = initial_pos;
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            match grid.get(r, c).unwrap() {
                '#' | '^' | '.' => continue,
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
            dir = turn_right(dir);
        } else {
            *c = dir_to_char(c.to_owned(), dir); // Just for visual debugging, not needed
            if visited.contains(&(row, col, dir)) {
                return true;
            }
            visited.insert((row, col, dir));
            (previous_row, previous_col) = (row, col);
        }

        if let (Some(r), Some(c)) = step_forward(row, col, dir) {
            (row, col) = (r, c);
        } else {
            return false;
        }
    }
    false
}

#[warn(dead_code)]
fn print_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        println!("{}", row.collect::<String>());
    }
}

const fn step_forward(row: usize, col: usize, dir: Direction) -> (Option<usize>, Option<usize>) {
    match (row, col, dir) {
        (0, _, Direction::Up) | (_, 0, Direction::Left) => (None, None),
        (r, c, Direction::Up) => (Some(r - 1), Some(c)),
        (r, c, Direction::Down) => (Some(r + 1), Some(c)),
        (r, c, Direction::Left) => (Some(r), Some(c - 1)),
        (r, c, Direction::Right) => (Some(r), Some(c + 1)),
    }
}

const fn dir_to_char(current: char, dir: Direction) -> char {
    match (dir, current) {
        (Direction::Up, '-')
        | (Direction::Down, '-')
        | (Direction::Left, '|')
        | (Direction::Right, '|') => '+',
        (Direction::Up, _) | (Direction::Down, _) => '|',
        (Direction::Left, _) | (Direction::Right, _) => '-',
    }
}

const fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
