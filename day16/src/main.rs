use grid::Grid;
use pathfinding::prelude::dijkstra;
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
    part1(PATH);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<char>>()).unwrap();

    let mut end = None;
    let mut start = None;
    for ((r, c), &val) in grid.indexed_iter() {
        if val == 'E' {
            end = Some((r, c));
        } else if val == 'S' {
            start = Some((r, c));
        }
    }

    let start_node = Node {
        cell: start.unwrap(),
        direction: Direction::East,
    };
    let end_cell = end.unwrap();

    let result = dijkstra(
        &start_node,
        |n| successors(&grid, n),
        |n| n.cell == end_cell,
    );

    println!("{}", result.unwrap().1);
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
        if c == '.' || c == 'E' {
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
            direction: turn_left(&node.direction),
        },
        1000usize,
    ));
    res.push((
        Node {
            cell: node.cell,
            direction: turn_right(&node.direction),
        },
        1000usize,
    ));

    // println!("Successors for {:?} {:?}", node.cell, node.direction);
    // println!("{:?}", res);

    res
}

fn turn_left(dir: &Direction) -> Direction {
    match dir {
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::North => Direction::West,
    }
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::North => Direction::East,
    }
}
