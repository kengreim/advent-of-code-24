#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use pathfinding::prelude::{astar_bag_collect, dijkstra};
use std::collections::HashSet;
use std::hash::Hash;
use utils::GridExt;

type Cell = (usize, usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    pub cell: Cell,
    pub direction: Direction,
    //pub cost: usize,
    //pub visited: bool,
}

// impl Node {
//     pub fn mark_visited(&mut self) {
//         self.visited = true;
//     }
//
//     pub fn set_cost(&mut self, cost: usize) {
//         self.cost = cost;
//     }
// }

// impl Hash for Node {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.cell.hash(state);
//         self.direction.hash(state);
//     }
// }

fn main() {
    const PATH: &str = "day16/src/day16_input.txt";
    let shortest_cost = part1(PATH);

    println!("{shortest_cost}");

    part2(PATH);
}

fn part1(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<char>>()).unwrap();

    let (start, end) = find_start_and_end(&grid).unwrap();
    let start_node = Node {
        cell: start,
        direction: Direction::East,
    };
    let end_cell = end;

    let result = dijkstra(
        &start_node,
        |n| successors(&grid, n),
        |n| n.cell == end_cell,
    );
    result.unwrap().1
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<char>>()).unwrap();

    let (start, end) = find_start_and_end(&grid).unwrap();
    let start_node = Node {
        cell: start,
        direction: Direction::East,
    };
    let end_cell = end;

    let paths = astar_bag_collect(
        &start_node,
        |n| successors(&grid, n),
        |n| end_cell.0.abs_diff(n.cell.0) + end_cell.1.abs_diff(n.cell.1),
        |n| n.cell == end_cell,
    );

    let all_cells = paths
        .unwrap()
        .0
        .iter()
        .flatten()
        .map(|p| p.cell)
        .collect::<HashSet<_>>();

    println!("{}", all_cells.len());
}

fn find_start_and_end(grid: &Grid<char>) -> Option<(Cell, Cell)> {
    let mut end = None;
    let mut start = None;
    for ((r, c), &val) in grid.indexed_iter() {
        if val == 'E' {
            end = Some((r, c));
        } else if val == 'S' {
            start = Some((r, c));
        }
    }
    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

fn successors(grid: &Grid<char>, node: &Node) -> Vec<(Node, usize)> {
    let (row, col) = node.cell;
    let mut res = vec![];

    let (next_row, next_col) = match node.direction {
        Direction::East => (row, col + 1),
        Direction::South => (row + 1, col),
        Direction::West => (row, col - 1),
        Direction::North => (row - 1, col),
    };

    if let Some(&c) = grid.get(next_row, next_col) {
        if c != '#' {
            res.push((
                Node {
                    cell: (next_row, next_col),
                    direction: node.direction,
                },
                1usize,
            ));
        }
    }

    res.push((
        Node {
            cell: node.cell,
            direction: turn_left(node.direction),
        },
        1000usize,
    ));
    res.push((
        Node {
            cell: node.cell,
            direction: turn_right(node.direction),
        },
        1000usize,
    ));

    res
}

const fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::North => Direction::West,
    }
}

const fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::North => Direction::East,
    }
}
