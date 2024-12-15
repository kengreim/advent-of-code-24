use grid::Grid;
use utils::GridExt;

fn main() {
    const PATH: &str = "day15/src/day15_input.txt";
    part1(PATH);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();

    let (mut grid, mut pos) = parse_grid(&input).unwrap();
    let moves = parse_moves(&input);

    for m in moves {
        let (next_cell, next_val) = find_next(&grid, pos, m);
        if let Some(c) = next_val {
            if c == '#' {
                continue;
            } else if c == '.' {
                grid[next_cell] = '@';
                grid[pos] = '.';
                pos = next_cell;
            } else if c == 'O' {
                let (delta_row, delta_col) = move_to_delta(m);
                if let Some(free_pos) = next_free(&grid, next_cell, (delta_row, delta_col)) {
                    grid[free_pos] = 'O';
                    grid[next_cell] = '@';
                    grid[pos] = '.';
                    pos = next_cell;
                }
            }
        } else {
            panic!()
        }
    }

    grid.print();
    println!("{}", score_grid(&grid));
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
    if let Some(c) = grid.get(row, col) {
        if *c == '.' {
            Some((row, col))
        } else if *c == '#' {
            None
        } else {
            next_free(
                grid,
                (
                    (row as isize + delta_row) as usize,
                    (col as isize + delta_col) as usize,
                ),
                (delta_row, delta_col),
            )
        }
    } else {
        unreachable!();
    }
}

fn parse_grid(input: &str) -> Option<(Grid<char>, (usize, usize))> {
    let lines = input
        .lines()
        .filter(|l| l.starts_with('#'))
        .collect::<Vec<_>>();

    let num_cols = lines.get(0)?.trim().chars().count();

    let grid = Grid::from_vec(
        lines
            .into_iter()
            .map(|l| l.trim().chars())
            .flatten()
            .collect(),
        num_cols,
    );

    let mut start_pos = (0usize, 0usize);
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid[(r, c)] == '@' {
                start_pos = (r, c);
                break;
            }
        }
    }

    Some((grid, start_pos))
}

fn parse_moves(input: &str) -> Vec<char> {
    input
        .lines()
        .filter(|l| {
            l.starts_with('^') || l.starts_with('v') || l.starts_with('<') || l.starts_with('>')
        })
        .map(|l| l.trim().chars())
        .flatten()
        .collect::<Vec<_>>()
}
