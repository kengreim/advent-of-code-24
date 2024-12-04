#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::fs;

fn main() {
    const PATH: &str = "day4/src/day4_input.txt";

    // Part 1
    let input = fs::read_to_string(PATH).unwrap();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            sum += check_xmas_part1(&matrix, i, j);
        }
    }
    println!("{sum}");

    // Part 2
    let mut sum2 = 0;
    for i in 1..rows {
        for j in 1..cols {
            sum2 += check_xmas_part2(&matrix, i, j);
        }
    }
    println!("{sum2}");
}

fn check_xmas_part1(matrix: &[Vec<char>], row: usize, column: usize) -> i32 {
    let blank = vec![];

    if matrix.get(row).unwrap()[column] != 'X' {
        0
    } else {
        let forward = (
            matrix.get(row).unwrap().get(column + 1).unwrap_or(&' '),
            matrix.get(row).unwrap().get(column + 2).unwrap_or(&' '),
            matrix.get(row).unwrap().get(column + 3).unwrap_or(&' '),
        );
        let backward = if column >= 3 {
            (
                matrix.get(row).unwrap().get(column - 1).unwrap_or(&' '),
                matrix.get(row).unwrap().get(column - 2).unwrap_or(&' '),
                matrix.get(row).unwrap().get(column - 3).unwrap_or(&' '),
            )
        } else {
            (&' ', &' ', &' ')
        };

        let up = if row >= 3 {
            (
                matrix
                    .get(row - 1)
                    .unwrap_or(&blank)
                    .get(column)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 2)
                    .unwrap_or(&blank)
                    .get(column)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 3)
                    .unwrap_or(&blank)
                    .get(column)
                    .unwrap_or(&' '),
            )
        } else {
            (&' ', &' ', &' ')
        };
        let down = (
            matrix
                .get(row + 1)
                .unwrap_or(&blank)
                .get(column)
                .unwrap_or(&' '),
            matrix
                .get(row + 2)
                .unwrap_or(&blank)
                .get(column)
                .unwrap_or(&' '),
            matrix
                .get(row + 3)
                .unwrap_or(&blank)
                .get(column)
                .unwrap_or(&' '),
        );
        let nw = if row >= 3 && column >= 3 {
            (
                matrix
                    .get(row - 1)
                    .unwrap_or(&blank)
                    .get(column - 1)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 2)
                    .unwrap_or(&blank)
                    .get(column - 2)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 3)
                    .unwrap_or(&blank)
                    .get(column - 3)
                    .unwrap_or(&' '),
            )
        } else {
            (&' ', &' ', &' ')
        };
        let ne = if row >= 3 {
            (
                matrix
                    .get(row - 1)
                    .unwrap_or(&blank)
                    .get(column + 1)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 2)
                    .unwrap_or(&blank)
                    .get(column + 2)
                    .unwrap_or(&' '),
                matrix
                    .get(row - 3)
                    .unwrap_or(&blank)
                    .get(column + 3)
                    .unwrap_or(&' '),
            )
        } else {
            (&' ', &' ', &' ')
        };
        let sw = if column >= 3 {
            (
                matrix
                    .get(row + 1)
                    .unwrap_or(&blank)
                    .get(column - 1)
                    .unwrap_or(&' '),
                matrix
                    .get(row + 2)
                    .unwrap_or(&blank)
                    .get(column - 2)
                    .unwrap_or(&' '),
                matrix
                    .get(row + 3)
                    .unwrap_or(&blank)
                    .get(column - 3)
                    .unwrap_or(&' '),
            )
        } else {
            (&' ', &' ', &' ')
        };
        let se = (
            matrix
                .get(row + 1)
                .unwrap_or(&blank)
                .get(column + 1)
                .unwrap_or(&' '),
            matrix
                .get(row + 2)
                .unwrap_or(&blank)
                .get(column + 2)
                .unwrap_or(&' '),
            matrix
                .get(row + 3)
                .unwrap_or(&blank)
                .get(column + 3)
                .unwrap_or(&' '),
        );

        let all = vec![forward, backward, up, down, nw, ne, sw, se];
        all.iter().fold(0, |acc, x| {
            acc + if x == &(&'M', &'A', &'S') { 1 } else { 0 }
        })
    }
}

fn check_xmas_part2(matrix: &[Vec<char>], row: usize, column: usize) -> i32 {
    let blank = vec![];
    if matrix.get(row).unwrap()[column] != 'A' {
        0
    } else {
        let cross1 = (
            matrix
                .get(row - 1)
                .unwrap_or(&blank)
                .get(column - 1)
                .unwrap_or(&' '),
            matrix
                .get(row)
                .unwrap_or(&blank)
                .get(column)
                .unwrap_or(&' '),
            matrix
                .get(row + 1)
                .unwrap_or(&blank)
                .get(column + 1)
                .unwrap_or(&' '),
        );
        let cross2 = (
            matrix
                .get(row - 1)
                .unwrap_or(&blank)
                .get(column + 1)
                .unwrap_or(&' '),
            matrix
                .get(row)
                .unwrap_or(&blank)
                .get(column)
                .unwrap_or(&' '),
            matrix
                .get(row + 1)
                .unwrap_or(&blank)
                .get(column - 1)
                .unwrap_or(&' '),
        );
        if (cross1 == (&'M', &'A', &'S') || cross1 == (&'S', &'A', &'M'))
            && (cross2 == (&'M', &'A', &'S') || cross2 == (&'S', &'A', &'M'))
        {
            1
        } else {
            0
        }
    }
}
