use itertools::Itertools;
use pathfinding::prelude::astar;
use std::cmp::min;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    part1();
}

fn part1() {
    let inputs = vec!["279A", "341A", "459A", "540A", "085A"];

    let start = Instant::now();
    for i in inputs {
        let robot1 = all_shortest_paths_for_sequence(i, true);
        let mut robot2 = robot1
            .iter()
            .map(|s| all_shortest_paths_for_sequence(s, false))
            .flatten()
            .collect::<Vec<_>>();

        let mut robot3 = robot2
            .iter()
            .map(|s| all_shortest_paths_for_sequence(s, false))
            .flatten()
            .collect::<Vec<_>>();

        //robot2.sort_by_key(|s| s.len());

        //println!("{:?}", robot2.len());

        // let robot3 = all_shortest_paths_for_sequence(robot2.first().unwrap(), false);
        // let mut min = usize::MAX;
        // for s in &robot3 {
        //     if s.len() < min {
        //         min = s.len();
        //     }
        // }

        //let robot3 = all_shortest_paths_for_sequence(robot2.first().unwrap(), false);

        let mut min = usize::MAX;
        for s in &robot3 {
            if s.len() < min {
                min = s.len();
            }
        }

        println!("{i} {min}"); // robot3.first().unwrap().len());
    }
    println!("{:?}", start.elapsed());
}

fn numpad_shortest_paths(current_char: char, next_char: char) -> HashSet<String> {
    shortest_paths(numpad_position(current_char), numpad_position(next_char))
}

fn direction_shortest_paths(current_char: char, next_char: char) -> HashSet<String> {
    shortest_paths(
        direction_position(current_char),
        direction_position(next_char),
    )
}

fn all_shortest_paths_for_sequence(output: &str, is_numpad: bool) -> Vec<String> {
    let mut paths: Vec<String> = vec!["".to_string()];

    let mut current = 'A';
    for next in output.chars() {
        let mut all_new = vec![];
        for p in &paths {
            let new_path = if is_numpad {
                numpad_shortest_paths(current, next)
            } else {
                direction_shortest_paths(current, next)
            };
            for new_step in new_path {
                let mut new = p.clone();
                new.push_str(&new_step);
                new.push('A');
                all_new.push(new);
            }
        }
        current = next;
        paths = all_new;
    }

    paths
}

fn shortest_paths(current: (i8, i8), next: (i8, i8)) -> HashSet<String> {
    let (current_r, current_c) = current;
    let (next_r, next_c) = next;

    if current_r == next_r && current_c == next_c {
        return HashSet::from_iter(vec!["".to_string()]);
    }

    let row_char = if current_r > next_r { 'v' } else { '^' };
    let col_char = if current_c > next_c { '<' } else { '>' };

    if current_c == next_c {
        return HashSet::from_iter(vec![repeat(row_char)
            .take(next_r.abs_diff(current_r) as usize)
            .collect::<String>()]);
    }

    if current_r == next_r {
        return HashSet::from_iter(vec![repeat(col_char)
            .take(next_c.abs_diff(current_c) as usize)
            .collect::<String>()]);
    }

    let mut chars = vec![];
    for _ in 0..next_r.abs_diff(current_r) {
        chars.push(row_char);
    }
    for _ in 0..next_c.abs_diff(current_c) {
        chars.push(col_char);
    }
    let size = next_r.abs_diff(current_r) + next_c.abs_diff(current_c);

    chars
        .iter()
        .permutations(size as usize)
        .map(|s| s.into_iter().collect::<String>())
        .collect::<HashSet<_>>()
}

fn numpad_position(n: char) -> (i8, i8) {
    match n {
        '0' => (0, 1),
        'A' => (0, 2),
        '1' => (1, 0),
        '2' => (1, 1),
        '3' => (1, 2),
        '4' => (2, 0),
        '5' => (2, 1),
        '6' => (2, 2),
        '7' => (3, 0),
        '8' => (3, 1),
        '9' => (3, 2),
        _ => panic!(),
    }
}

fn direction_position(n: char) -> (i8, i8) {
    match n {
        '<' => (0, 0),
        'v' => (0, 1),
        '>' => (0, 2),
        '^' => (1, 1),
        'A' => (1, 2),
        _ => panic!(),
    }
}
