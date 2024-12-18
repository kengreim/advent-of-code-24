use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::fs;

fn main() {
    const PATH: &str = "day18/src/day18_input.txt";

    part1(PATH);
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let fallen_all = input.lines().filter_map(|x| split_line(x));

    let fallen = fallen_all.take(1024).collect::<HashSet<_>>();

    let max_row = 71;
    let max_col = 71;

    let x = astar(
        &(0, 0),
        |(r, c)| successors((*r, *c), (max_row, max_col), &fallen),
        |(r, c)| r.abs_diff(max_row) + c.abs_diff(max_col),
        |(r, c)| *r == max_col - 1 && *c == max_col - 1,
    );

    println!("{}", x.unwrap().0.len() - 1)
}

fn successors(
    (r, c): (u32, u32),
    (max_r, max_c): (u32, u32),
    fallen_bytes: &HashSet<(u32, u32)>,
) -> Vec<((u32, u32), u32)> {
    let mut result = Vec::new();
    if r > 0 && !fallen_bytes.contains(&(r - 1, c)) {
        result.push(((r - 1, c), 1));
    }
    if c > 0 && !fallen_bytes.contains(&(r, c - 1)) {
        result.push(((r, c - 1), 1));
    }

    if r + 1 < max_r && !fallen_bytes.contains(&(r + 1, c)) {
        result.push(((r + 1, c), 1));
    }

    if c + 1 < max_c && !fallen_bytes.contains(&(r, c + 1)) {
        result.push(((r, c + 1), 1));
    }

    result
}

fn split_line(line: &str) -> Option<(u32, u32)> {
    let l = line.trim().split(',').collect::<Vec<&str>>();
    Some((l[0].parse::<u32>().ok()?, l[1].parse::<u32>().ok()?))
}
