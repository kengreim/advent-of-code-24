#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use grid::Grid;
use std::cmp::min;
use utils::GridExt;

type WideCell = (usize, (usize, usize));

fn main() {
    const PATH: &str = "day15/src/day15_test.txt";
    //part1(PATH);
    part2(PATH);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let (mut grid, mut pos) = parse_grid(&input).unwrap();

    for m in parse_moves(&input) {
        let (next_cell, next_val) = find_next(&grid, pos, m);
        match next_val.unwrap() {
            '#' => continue,
            '.' => {
                grid[next_cell] = '@';
                grid[pos] = '.';
                pos = next_cell;
            }
            'O' => {
                let delta = move_to_delta(m);
                if let Some(free_pos) = next_free(&grid, next_cell, delta) {
                    grid[free_pos] = 'O';
                    grid[next_cell] = '@';
                    grid[pos] = '.';
                    pos = next_cell;
                }
            }
            _ => panic!(),
        }
    }

    grid.print();
    println!("{}", score_grid(&grid));
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();

    let (original_grid, _) = parse_grid(&input).unwrap();
    let (mut grid, mut pos) = widen_grid(&original_grid);

    #[cfg(debug_assertions)]
    grid.print();

    for m in parse_moves(&input) {
        #[cfg(debug_assertions)]
        println!("Move {m}");

        let (next_cell, next_val) = find_next(&grid, pos, m);
        match next_val.unwrap() {
            '#' => (), // Not using continue so that we can print in debug before next iteration
            '.' => {
                grid[next_cell] = '@';
                grid[pos] = '.';
                pos = next_cell;
            }
            '[' | ']' => match m {
                '<' | '>' => {
                    let delta = move_to_delta(m);
                    if let Some((_, free_col)) = next_free(&grid, next_cell, delta) {
                        let (next_row, next_col) = next_cell;
                        let reverse = free_col > next_col;

                        let mut moving_char = if free_col < next_col { '[' } else { ']' };
                        let mut move_col = free_col;
                        while move_col != next_col {
                            grid[(next_row, move_col)] = moving_char;
                            moving_char = if moving_char == '[' { ']' } else { '[' };
                            move_col = if reverse { move_col - 1 } else { move_col + 1 };
                        }
                        grid[next_cell] = '@';
                        grid[pos] = '.';
                        pos = next_cell;
                    }
                }
                '^' | 'v' => {
                    let (next_row, next_col) = next_cell;
                    let mut left_aligned: bool;
                    let mut widebox: WideCell;

                    if next_val.unwrap() == '[' {
                        widebox = (next_row, (next_col, next_col + 1));
                        left_aligned = true;
                    } else {
                        widebox = (next_row, (next_col - 1, next_col));
                        left_aligned = false;
                    }

                    // let (min_check_col, max_check_col) = if next_val.unwrap() == '[' {
                    //     (next_col, next_col + 1)
                    // } else {
                    //     (next_col - 1, next_col)
                    // };
                    //println!("{min_check_col} {max_check_col}");

                    let is_increasing_row = m == 'v';

                    let test_grid = grid.clone();

                    if let Some(mut boxes) =
                        next_free_wide(&grid, next_cell, is_increasing_row, vec![widebox], vec![])
                    {
                        println!("Found some boxes {:?}", boxes);
                        if is_increasing_row {
                            boxes.sort_by(|a, b| b.0.cmp(&a.0));
                        } else {
                            boxes.sort_by(|a, b| a.0.cmp(&b.0));
                        }

                        for (row, (c1, c2)) in boxes {
                            let offset_row = if is_increasing_row { row - 1 } else { row + 1 };

                            //grid.print();
                            //println!("writing {offset_row} {c1} {c2} to {row} {c1} {c2}");
                            grid[(row, c1)] = grid[(offset_row, c1)];
                            grid[(row, c2)] = grid[(offset_row, c2)];
                        }

                        // let mut move_row = free_row;
                        // while move_row != next_row {
                        //     let offset_row = if is_increasing_row {
                        //         move_row - 1
                        //     } else {
                        //         move_row + 1
                        //     };
                        //     for col in min_col_free..=max_col_free {
                        //         println!("{move_row} {col} comes from {offset_row} {col}");
                        //         grid[(move_row, col)] = grid[(offset_row, col)];
                        //     }
                        //
                        //     move_row = if is_increasing_row {
                        //         move_row - 1
                        //     } else {
                        //         move_row + 1
                        //     };
                        // }

                        //println!("{:?}", next_cell);
                        // if grid[next_cell] == '@' {
                        //     grid[(next_row, next_col + 1)] = '.';
                        // } else {
                        //     grid[(next_row, next_col + 1)] = '.';
                        // }

                        grid[next_cell] = '@';
                        if left_aligned {
                            grid[(next_row, next_col + 1)] = '.';
                        } else {
                            grid[(next_row, next_col - 1)] = '.';
                        }
                        grid[pos] = '.';
                        pos = next_cell;
                    }
                    test_grid.print();
                    println!("Move {m}");
                    grid.print();
                }
                _ => panic!(),
            },
            _ => panic!(),
        }

        #[cfg(debug_assertions)]
        grid.print();
    }
}

fn score_grid(grid: &Grid<char>) -> usize {
    grid.indexed_iter()
        .map(|((row, col), c)| if *c == 'O' { 100 * row + col } else { 0 })
        .sum()
}

fn move_to_delta(move_char: char) -> (isize, isize) {
    match move_char {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid move char: {move_char}"),
    }
}

fn find_next(
    grid: &Grid<char>,
    (row, col): (usize, usize),
    move_char: char,
) -> ((usize, usize), Option<char>) {
    let (delta_row, delta_col) = move_to_delta(move_char);
    let (new_row, new_col) = (
        (row as isize + delta_row) as usize,
        (col as isize + delta_col) as usize,
    );
    ((new_row, new_col), grid.get(new_row, new_col).copied())
}

fn next_free(
    grid: &Grid<char>,
    (row, col): (usize, usize),
    (delta_row, delta_col): (isize, isize),
) -> Option<(usize, usize)> {
    grid.get(row, col).map_or_else(
        || panic!("Invalid at {row} {col}"),
        |c| match c {
            '#' => None,
            '.' => Some((row, col)),
            _ => next_free(
                grid,
                (
                    (row as isize + delta_row) as usize,
                    (col as isize + delta_col) as usize,
                ),
                (delta_row, delta_col),
            ),
        },
    )
}

fn next_free_wide(
    grid: &Grid<char>,
    (row, col): (usize, usize),
    is_increasing_row: bool,
    //(mut min_check_col, mut max_check_col): (usize, usize),
    frontier_row: Vec<WideCell>,
    mut acc_boxes: Vec<WideCell>,
) -> Option<Vec<WideCell>> {
    if frontier_row.iter().any(|(row, (col1, col2))| {
        *grid.get(*row, *col1).unwrap() == '#' || *grid.get(*row, *col2).unwrap() == '#'
    }) {
        None
    } else if frontier_row.iter().all(|(row, (col1, col2))| {
        *grid.get(*row, *col1).unwrap() == '.' && *grid.get(*row, *col2).unwrap() == '.'
    }) {
        acc_boxes.extend(frontier_row);
        Some(acc_boxes)
    } else {
        acc_boxes.extend(frontier_row.clone());

        let mut new_frontier = vec![];

        let next_row = if is_increasing_row { row + 1 } else { row - 1 };

        for (_, (col1, col2)) in frontier_row {
            new_frontier.push((next_row, (col1, col2)));
            //println!("{next_row} {col1} {col2}");
            if *grid.get(next_row, col1).unwrap() == ']' {
                new_frontier.push((next_row, (col1 - 1, col1)))
            }
            if *grid.get(next_row, col2).unwrap() == '[' {
                new_frontier.push((next_row, (col2, col2 + 1)))
            }
        }

        next_free_wide(
            grid,
            (next_row, col),
            is_increasing_row,
            new_frontier,
            acc_boxes,
        )
    }

    // let chars = (min_check_col..=max_check_col)
    //     .filter_map(|col| grid.get(row, col).copied())
    //     .collect::<Vec<_>>();
    //
    // if chars.iter().any(|c| *c == '#') {
    //     None
    // } else if chars.iter().all(|c| *c == '.') {
    //     Some(((row, min_check_col), (row, max_check_col)))
    // } else {
    //     let next_row = if is_increasing_row { row + 1 } else { row - 1 };
    //     min_check_col = if *chars.first().unwrap() == ']' {
    //         min_check_col - 1
    //     } else {
    //         min_check_col
    //     };
    //     max_check_col = if *chars.last().unwrap() == '[' {
    //         max_check_col + 1
    //     } else {
    //         max_check_col
    //     };
    //     next_free_wide(
    //         grid,
    //         (next_row, col),
    //         is_increasing_row,
    //         (min_check_col, max_check_col),
    //     )
    // }

    // loop {
    //     let chars = (min_check_col..=max_check_col)
    //         .filter_map(|col| grid.get(check_row, col).copied())
    //         .collect::<Vec<_>>();
    //
    //     if chars.iter().any(|c| *c == '#') {
    //         return if check_row == row {
    //             None
    //         } else {
    //             Some(((check_row, min_check_col), (check_row, max_check_col)))
    //         };
    //     } else if chars.iter().all(|c| *c == '.') {
    //         return Some(((row, min_check_col), (row, max_check_col)));
    //     } else if *chars.first().unwrap() == ']' {
    //         min_check_col -= 1;
    //     } else if *chars.last().unwrap() == '[' {
    //         max_check_col += 1;
    //     }
    //
    //     check_row = if is_increasing_row {
    //         check_row + 1
    //     } else {
    //         check_row - 1
    //     };
    // }
}

fn parse_grid(input: &str) -> Option<(Grid<char>, (usize, usize))> {
    let lines = input
        .lines()
        .filter(|l| l.starts_with('#'))
        .collect::<Vec<_>>();

    let num_cols = lines.first()?.trim().chars().count();

    let grid = Grid::from_vec(
        lines.into_iter().flat_map(|l| l.trim().chars()).collect(),
        num_cols,
    );

    let mut start_pos = (0usize, 0usize);
    for ((row, col), c) in grid.indexed_iter() {
        if *c == '@' {
            start_pos = (row, col);
            break;
        }
    }

    Some((grid, start_pos))
}

fn parse_moves(input: &str) -> impl Iterator<Item = char> + use<'_> {
    input
        .lines()
        .filter(|l| {
            l.starts_with('^') || l.starts_with('v') || l.starts_with('<') || l.starts_with('>')
        })
        .flat_map(|l| l.trim().chars())
}

fn widen_grid(grid: &Grid<char>) -> (Grid<char>, (usize, usize)) {
    let mut new_grid = Grid::new(grid.rows(), grid.cols() * 2);
    let mut start_pos = (0usize, 0usize);
    for ((row, col), c) in grid.indexed_iter() {
        match c {
            '.' | '#' => {
                new_grid[(row, col * 2)] = *c;
                new_grid[(row, col * 2 + 1)] = *c;
            }
            'O' => {
                new_grid[(row, col * 2)] = '[';
                new_grid[(row, col * 2 + 1)] = ']';
            }
            '@' => {
                new_grid[(row, col * 2)] = '@';
                new_grid[(row, col * 2 + 1)] = '.';
                start_pos = (row, col * 2);
            }
            _ => panic!("Invalid grid char: {c}"),
        }
    }

    (new_grid, start_pos)
}
