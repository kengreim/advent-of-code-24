use itertools::Itertools;
use pathfinding::prelude::astar;
use std::cmp::min;
use std::hash::Hash;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    part1();
}

#[derive(PartialEq, Eq, Clone)]
struct NumpadNode {
    pub state: Option<char>,
    pub output: String,
    pub sequence: Vec<char>,
}

#[derive(PartialEq, Eq, Clone)]
struct DirectionNode {
    pub state: Option<char>,
    pub output: String,
    pub sequence: Vec<char>,
}

impl Hash for DirectionNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.output.hash(state);
        self.state.hash(state);
    }
}

impl Hash for NumpadNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.output.hash(state);
        self.state.hash(state);
    }
}

impl Default for NumpadNode {
    fn default() -> Self {
        Self {
            state: Some('A'),
            output: String::new(),
            sequence: Vec::new(),
        }
    }
}

impl Default for DirectionNode {
    fn default() -> Self {
        Self {
            state: Some('A'),
            output: String::new(),
            sequence: Vec::new(),
        }
    }
}

trait ExecuteNode {
    fn evaluate_last(&mut self) -> String;
    fn get_state(&self) -> Option<char>;
    fn get_sequence(&self) -> &[char];
    fn get_output(&self) -> &str;
}

impl ExecuteNode for NumpadNode {
    fn evaluate_last(&mut self) -> String {
        let &s = self.sequence.last().unwrap();
        match s {
            '<' => {
                self.state = match self.state {
                    Some('9') => Some('8'),
                    Some('8') => Some('7'),
                    Some('6') => Some('5'),
                    Some('5') => Some('4'),
                    Some('3') => Some('2'),
                    Some('2') => Some('1'),
                    Some('A') => Some('0'),
                    Some('0') => None,
                    _ => self.state,
                };
            }
            '>' => {
                self.state = match self.state {
                    Some('8') => Some('9'),
                    Some('7') => Some('8'),
                    Some('5') => Some('6'),
                    Some('4') => Some('5'),
                    Some('2') => Some('3'),
                    Some('1') => Some('2'),
                    Some('0') => Some('A'),
                    _ => self.state,
                };
            }
            '^' => {
                self.state = match self.state {
                    Some('A') => Some('3'),
                    Some('3') => Some('6'),
                    Some('6') => Some('9'),
                    Some('0') => Some('2'),
                    Some('2') => Some('5'),
                    Some('5') => Some('8'),
                    Some('1') => Some('4'),
                    Some('4') => Some('7'),
                    _ => self.state,
                };
            }
            'v' => {
                self.state = match self.state {
                    Some('3') => Some('A'),
                    Some('6') => Some('3'),
                    Some('9') => Some('6'),
                    Some('2') => Some('0'),
                    Some('5') => Some('2'),
                    Some('8') => Some('5'),
                    Some('4') => Some('1'),
                    Some('7') => Some('4'),
                    Some('1') => None,
                    _ => self.state,
                };
            }
            'A' => self.output.push(self.state.unwrap()),
            _ => panic!(),
        }
        self.output.clone()
    }

    fn get_state(&self) -> Option<char> {
        self.state
    }

    fn get_sequence(&self) -> &[char] {
        &self.sequence
    }

    fn get_output(&self) -> &str {
        &self.output
    }
}

impl ExecuteNode for DirectionNode {
    fn evaluate_last(&mut self) -> String {
        let &s = self.sequence.last().unwrap();
        match s {
            '<' => {
                self.state = match self.state {
                    Some('A') => Some('^'),
                    Some('>') => Some('v'),
                    Some('v') => Some('<'),
                    Some('^') => None,
                    _ => self.state,
                };
            }
            '>' => {
                self.state = match self.state {
                    Some('^') => Some('A'),
                    Some('<') => Some('v'),
                    Some('v') => Some('>'),
                    _ => self.state,
                };
            }
            '^' => {
                self.state = match self.state {
                    Some('v') => Some('^'),
                    Some('>') => Some('A'),
                    Some('<') => None,
                    _ => self.state,
                };
            }
            'v' => {
                self.state = match self.state {
                    Some('^') => Some('v'),
                    Some('A') => Some('>'),
                    _ => self.state,
                };
            }
            'A' => self.output.push(self.state.unwrap()),
            _ => panic!(),
        }
        self.output.clone()
    }

    fn get_state(&self) -> Option<char> {
        self.state
    }

    fn get_sequence(&self) -> &[char] {
        &self.sequence
    }

    fn get_output(&self) -> &str {
        &self.output
    }
}

fn part1() {
    let output = "029A";

    let output_fixed = output
        .chars()
        .interleave(repeat('A').take(output.len()))
        .collect::<Vec<_>>();

    println!("{:?}", output_fixed);
    let mut paths: Vec<String> = vec!["".to_string()];

    for next in output_fixed {
        let mut all_new = vec![];
        for p in &paths {
            let last = p.chars().last().unwrap_or('A');
            let new_path = numpad_shortest_path(last, next);
            println!("{} {}", last, next);
            println!("{:?}", new_path);
            for new_step in new_path {
                let mut new = p.clone();
                new.push_str(&new_step);
                all_new.push(new);
            }
        }
        paths = all_new;
    }

    println!("{:?}", paths);

    // let output = "029A";
    // let start = NumpadNode::default();
    // let start_t = Instant::now();
    // let x = astar(
    //     &start,
    //     |n| node_successors(n),
    //     |n| heuristic(n, output),
    //     |n| n.output == output,
    // );
    // println!("Part 1 took {:?}", start_t.elapsed());
    //
    // println!("{:?}", x.unwrap().0.last().unwrap().sequence);

    // let target = "<A^A^^>AvvvA";
    // let start2 = DirectionNode::default();
    // let x = astar(
    //     &start2,
    //     |n| node_successors_direction(n),
    //     |n| heuristic(n, target),
    //     |n| n.output == target,
    // );
    //
    // println!("{:?}", x.unwrap().0.last().unwrap().sequence)
}

fn heuristic(n: &impl ExecuteNode, target: &str) -> usize {
    if n.get_output() == "" {
        target.len()
    } else if !target.starts_with(n.get_output()) {
        1000
    } else {
        target.len() - max_prefix_len(target, n.get_output())
    }
}

fn node_successors(n: &impl ExecuteNode) -> Vec<(NumpadNode, usize)> {
    let directions = vec!['^', 'v', '<', '>', 'A'];
    let mut res = vec![];
    for d in directions {
        let mut new_seq = n.get_sequence().to_vec();
        new_seq.push(d);
        let mut new_node = NumpadNode {
            state: n.get_state(),
            output: n.get_output().to_string(),
            sequence: new_seq,
        };
        new_node.evaluate_last();
        if new_node.state.is_some() {
            res.push((new_node, 1));
        }
    }

    res
}

fn node_successors_direction(n: &impl ExecuteNode) -> Vec<(DirectionNode, usize)> {
    let directions = vec!['^', 'v', '<', '>', 'A'];
    let mut res = vec![];
    for d in directions {
        let mut new_seq = n.get_sequence().to_vec();
        new_seq.push(d);
        let mut new_node = DirectionNode {
            state: n.get_state(),
            output: n.get_output().to_string(),
            sequence: new_seq,
        };
        new_node.evaluate_last();
        if new_node.state.is_some() {
            res.push((new_node, 1));
            //println!("{}", new_node.get_output());
        }
    }

    res
}

fn max_prefix_len(s1: &str, s2: &str) -> usize {
    let mut res = 0;
    for i in 0..min(s1.len(), s2.len()) {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
            break;
        }
        res += 1
    }
    res
}

fn numpad_shortest_path(current_char: char, next_char: char) -> Vec<String> {
    let (current_r, current_c) = numpad_position(current_char);
    let (next_r, next_c) = numpad_position(next_char);

    if current_r == next_r && current_c == next_c {
        return vec!["".to_string()];
    }

    let row_char = if current_r > next_r { 'v' } else { '^' };
    let col_char = if current_c > next_c { '<' } else { 'v' };

    if current_c == next_c {
        return vec![repeat(row_char)
            .take(next_r.abs_diff(current_r) as usize)
            .collect::<String>()];
    }

    if current_r == next_r {
        return vec![repeat(col_char)
            .take(next_c.abs_diff(current_c) as usize)
            .collect::<String>()];
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
        .collect::<Vec<_>>()
}

fn numpad_position(n: char) -> (i8, i8) {
    match n {
        'A' => (0, 1),
        '0' => (0, 2),
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
