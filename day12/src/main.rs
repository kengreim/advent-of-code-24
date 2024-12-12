use grid::Grid;
use std::collections::{HashSet, VecDeque};
use std::fs;
use utils::GridExt;

fn main() {
    const PATH: &str = "day12/src/day12_input.txt";

    let input = fs::read_to_string(PATH).unwrap();
    let grid = Grid::parse_from_str(&input, |l| l.trim().chars().collect::<Vec<_>>()).unwrap();

    let mut visited = HashSet::new();
    let mut sum = 0;

    for ((r1, c1), char) in grid.indexed_iter() {
        if visited.contains(&(r1, c1)) {
            continue;
        }

        let mut current_region: HashSet<(usize, usize)> = HashSet::from_iter(vec![(r1, c1)]);
        let mut current_region_perim = 0;

        visited.insert((r1, c1));
        let mut queue = VecDeque::from(vec![(r1, c1)]);
        while !queue.is_empty() {
            let (r2, c2) = queue.pop_front().unwrap();

            let mut cell_perim = 4;
            for (idx, _) in grid.cardinal_neighbors_with((r2, c2), |val| *val == *char) {
                if !visited.contains(&idx) {
                    queue.push_back(idx);
                    visited.insert(idx);
                    current_region.insert(idx);
                }
                cell_perim -= 1;
            }
            current_region_perim += cell_perim;
        }
        sum += current_region.len() * current_region_perim;

        // println!("{:?}", current_region);
        // let area = current_region.len();
        // println!("area {}", area);
        // println!("perimeter {}", current_region_perim);
        // println!();
    }
    println!("sum: {}", sum);
}
