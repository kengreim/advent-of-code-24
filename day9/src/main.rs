#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::Sector::{File, Free};
use std::fmt::Display;
use std::fs;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part1();
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
    let (mut disk_expanded, mut first_free_idx, mut last_file_idx) = parse_disk(PATH);

    //print_disk_string(&disk_expanded);

    while first_free_idx < last_file_idx {
        let cloned = disk_expanded.clone();
        let avail_free = if let Free(n) = &cloned[first_free_idx] {
            n
        } else {
            panic!()
        };

        let (file_id, file_size) = if let File(id, size) = &cloned[last_file_idx] {
            (id, size)
        } else {
            panic!()
        };

        if avail_free >= file_size {
            *disk_expanded.get_mut(last_file_idx).unwrap() = Free(*file_size);

            if avail_free == file_size {
                *disk_expanded.get_mut(first_free_idx).unwrap() = File(*file_id, *file_size);
            } else {
                disk_expanded.insert(first_free_idx, File(*file_id, *file_size));
                *disk_expanded.get_mut(first_free_idx + 1).unwrap() = Free(avail_free - file_size);
            }
        } else {
            *disk_expanded.get_mut(first_free_idx).unwrap() = File(*file_id, *avail_free);
            *disk_expanded.get_mut(last_file_idx).unwrap() =
                File(*file_id, *file_size - avail_free);
            disk_expanded.push(Free(*avail_free));
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

    println!("{}", checksum(&disk_expanded))
}

fn part2() {
    const PATH: &str = "day9/src/day9_input.txt";
    let (mut disk_expanded, mut first_free_idx, mut last_file_idx) = parse_disk(PATH);

    println!("{}", checksum(&disk_expanded))
}

fn parse_disk(path: &str) -> (Vec<Sector>, usize, usize) {
    let disk_map = fs::read_to_string(path).unwrap();

    let mut disk_expanded: Vec<Sector> = vec![];
    let mut first_free_idx = None;
    let mut last_file_idx = None;

    let mut id = 0;
    for (i, c) in disk_map.chars().filter(|c| !c.is_whitespace()).enumerate() {
        if i % 2 == 0 {
            disk_expanded.push(File(id, c.to_digit(10).unwrap() as u8));
            id += 1;
            last_file_idx = Some(i);
        } else {
            disk_expanded.push(Free(c.to_digit(10).unwrap() as u8));
            if first_free_idx.is_none() {
                first_free_idx = Some(i);
            }
        }
    }

    (
        disk_expanded,
        first_free_idx.unwrap(),
        last_file_idx.unwrap(),
    )
}

fn checksum(sectors: &[Sector]) -> u64 {
    let mut checksum: u64 = 0;
    let mut block_count: u64 = 0;
    for sector in sectors {
        if let File(id, size) = sector {
            for _ in 0u8..*size {
                checksum += block_count * (*id as u64);
                block_count += 1;
            }
        } else {
            break;
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
