#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::Sector::{File, Free};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part2();
    println!("Elapsed time: {:.2?}", start.elapsed());
}

type Id = i32;
type Size = u8;

#[derive(Clone)]
enum Sector {
    File(Id, Size),
    Free(Size),
}

fn part1() {
    const PATH: &str = "day9/src/day9_input.txt";
    let (mut disk_expanded, (mut first_free_idx, _), (mut last_file_idx, _, _)) =
        parse_disk(PATH).unwrap();

    //print_disk_string(&disk_expanded);

    while first_free_idx < last_file_idx {
        let (avail_free, file_id, file_size) = match (
            disk_expanded[first_free_idx].clone(),
            disk_expanded[last_file_idx].clone(),
        ) {
            (Free(n), File(id, size)) => (n, id, size),
            _ => panic!(),
        };

        if avail_free >= file_size {
            *disk_expanded.get_mut(last_file_idx).unwrap() = Free(file_size);

            if avail_free == file_size {
                *disk_expanded.get_mut(first_free_idx).unwrap() = File(file_id, file_size);
            } else {
                disk_expanded.insert(first_free_idx, File(file_id, file_size));
                *disk_expanded.get_mut(first_free_idx + 1).unwrap() = Free(avail_free - file_size);
            }
        } else {
            *disk_expanded.get_mut(first_free_idx).unwrap() = File(file_id, avail_free);
            *disk_expanded.get_mut(last_file_idx).unwrap() = File(file_id, file_size - avail_free);
            disk_expanded.push(Free(avail_free));
        }

        for i in first_free_idx..disk_expanded.len() {
            if let Free(_) = disk_expanded[i] {
                first_free_idx = i;
                break;
            }
        }

        for i in (0..=last_file_idx).rev() {
            if let File(_, _) = disk_expanded[i] {
                last_file_idx = i;
                break;
            }
        }
    }

    println!("{}", checksum(&disk_expanded, true))
}

fn part2() {
    const PATH: &str = "day9/src/day9_input.txt";
    let (mut disk_expanded, (_, _), (mut last_file_idx, mut file_id, mut file_size)) =
        parse_disk(PATH).unwrap();

    let mut free_blocks = HashMap::new();

    // One try for each file_id, all the way down to 0
    while file_id > 0 {
        if let Some((found_free_idx, found_free_size, free_blocks_updated)) = find_first_free_with(
            &disk_expanded,
            free_blocks.clone(),
            |s| s >= file_size,
            last_file_idx,
        ) {
            free_blocks = free_blocks_updated;

            if found_free_size == file_size {
                *disk_expanded.get_mut(last_file_idx).unwrap() = Free(file_size);
                *disk_expanded.get_mut(found_free_idx).unwrap() = File(file_id, file_size);

                // If we only made a swap, tracked free indices don't change. Just remove previously tracked free
                free_blocks.remove(&found_free_idx);
            } else if found_free_size > file_size {
                *disk_expanded.get_mut(last_file_idx).unwrap() = Free(file_size);
                disk_expanded.insert(found_free_idx, File(file_id, file_size));
                *disk_expanded.get_mut(found_free_idx + 1).unwrap() =
                    Free(found_free_size - file_size);

                // We need to adjust all tracked free indices that are beyond found index, starting with the highest
                let mut keys = free_blocks.keys().cloned().collect::<Vec<_>>();
                keys.sort_unstable();
                for key_idx in keys.iter().rev() {
                    if *key_idx > found_free_idx {
                        let tmp = free_blocks.get(key_idx).unwrap();
                        free_blocks.insert(key_idx + 1, *tmp);
                        free_blocks.remove(&key_idx);
                    } else {
                        break;
                    }
                }
            }

            //print_disk_string(&disk_expanded);
        }

        // Decrement file_id and find index
        for i in (0..last_file_idx).rev() {
            if let File(next_file_id, next_file_size) = disk_expanded[i] {
                (file_id, file_size, last_file_idx) = (next_file_id, next_file_size, i);
                break;
            }
        }
    }
    println!("{}", checksum(&disk_expanded, false))
}

fn find_first_free_with(
    sectors: &[Sector],
    mut try_first_map: HashMap<usize, Size>,
    func: impl Fn(Size) -> bool,
    max_index: usize,
) -> Option<(usize, Size, HashMap<usize, Size>)> {
    let mut last_try_idx = 0;
    let mut sorted_keys = try_first_map.keys().cloned().collect::<Vec<_>>();
    sorted_keys.sort_unstable();

    for key_idx in sorted_keys {
        let try_size = try_first_map.get(&key_idx).unwrap();
        last_try_idx = key_idx;
        if key_idx < max_index && func(*try_size) {
            return Some((key_idx, *try_size, try_first_map));
        }
    }

    for idx in last_try_idx + 1..sectors.len() {
        if idx >= max_index {
            break;
        }
        if let Free(size) = sectors[idx] {
            if func(size) {
                return Some((idx, size, try_first_map));
            } else {
                try_first_map.insert(idx, size);
            }
        }
    }
    None
}

fn parse_disk(path: &str) -> Option<(Vec<Sector>, (usize, Size), (usize, Id, Size))> {
    let disk_map = fs::read_to_string(path).unwrap();

    let mut disk_expanded: Vec<Sector> = vec![];
    let mut first_free: Option<(usize, Sector)> = None;
    let mut last_file: Option<(usize, Sector)> = None;

    let mut id = 0;
    for (i, c) in disk_map.chars().filter(|c| !c.is_whitespace()).enumerate() {
        if i % 2 == 0 {
            let file = File(id, c.to_digit(10).unwrap() as u8);
            disk_expanded.push(file.clone());
            id += 1;
            last_file = Some((i, file));
        } else {
            let free = Free(c.to_digit(10).unwrap() as u8);
            disk_expanded.push(free.clone());
            if first_free.is_none() {
                first_free = Some((i, free));
            }
        }
    }

    match (first_free, last_file) {
        (Some((free_idx, Free(free_size))), Some((file_idx, File(file_id, file_size)))) => Some((
            disk_expanded,
            (free_idx, free_size),
            (file_idx, file_id, file_size),
        )),
        _ => None,
    }
}

fn checksum(sectors: &[Sector], break_on_first_free: bool) -> u64 {
    let mut checksum: u64 = 0;
    let mut block_count: u64 = 0;
    for sector in sectors {
        if let File(id, size) = sector {
            for _ in 0u8..*size {
                checksum += block_count * (*id as u64);
                block_count += 1;
            }
        } else if let Free(size) = sector {
            if break_on_first_free {
                break;
            }
            block_count += *size as u64;
        }
    }
    checksum
}

fn print_disk_string(sectors: &[Sector]) {
    let full_str = sectors.iter().map(|s| s.to_string()).collect::<String>();
    println!("{}", full_str);
}

impl Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Free(size) => {
                write!(
                    f,
                    "{}",
                    repeat('.').take(*size as usize).collect::<String>()
                )
            }
            File(id, size) => {
                write!(
                    f,
                    "{}",
                    repeat(id.to_string())
                        .take(*size as usize)
                        .collect::<String>()
                )
            }
        }
    }
}
