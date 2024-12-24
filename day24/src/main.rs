use petgraph::graph::DiGraph;
use petgraph::Direction;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::sync::LazyLock;

static WIRE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([a-z0-9]{3}): ([01])").unwrap());

static GATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap()
});

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Logic {
    OR,
    AND,
    XOR,
    UNKNOWN,
    NONE,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GateNode {
    name: String,
    logic: Logic,
    value: Option<bool>,
}

fn main() {
    const PATH: &str = "day24/src/day24_input.txt";
    part1(PATH);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();

    let mut map = HashMap::new();

    let mut g = DiGraph::new();

    // Load initial nodes
    for c in WIRE_RE.captures_iter(&input) {
        let node = c.get(1).unwrap().as_str();
        let val = c.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let new_node = GateNode {
            name: node.to_string(),
            logic: Logic::NONE,
            value: Some(val != 0),
        };
        let index = g.add_node(new_node);
        map.insert(node, index);
    }

    // Load all gate nodes
    for c in GATE_RE.captures_iter(&input) {
        let input_nodes = [c.get(1).unwrap().as_str(), c.get(3).unwrap().as_str()];
        let input_node_indices = input_nodes.map(|n| {
            if let Some(node) = map.get(n) {
                *node
            } else {
                let new_node = GateNode {
                    name: n.to_string(),
                    logic: Logic::UNKNOWN,
                    value: None,
                };
                let index = g.add_node(new_node);
                map.insert(n, index);
                index
            }
        });

        let output_node_str = c.get(4).unwrap().as_str();
        let logic = match c.get(2).unwrap().as_str() {
            "OR" => Logic::OR,
            "AND" => Logic::AND,
            "XOR" => Logic::XOR,
            _ => panic!(),
        };
        let output_node_index = if let Some(node) = map.get(output_node_str) {
            g[*node].logic = logic;
            *node
        } else {
            let new_node = GateNode {
                name: output_node_str.to_string(),
                logic,
                value: None,
            };
            let index = g.add_node(new_node);
            map.insert(output_node_str, index);
            index
        };

        g.add_edge(input_node_indices[0], output_node_index, ());
        g.add_edge(input_node_indices[1], output_node_index, ());
    }

    let mut queue = VecDeque::from_iter(g.node_indices());

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let inputs = g
            .neighbors_directed(node, Direction::Incoming)
            .collect::<Vec<_>>();

        match inputs.len() {
            0 => continue,
            2 => {
                let input1 = inputs[0];
                let input2 = inputs[1];

                if let (Some(val1), Some(val2)) = (g[input1].value, g[input2].value) {
                    let new_bool = match g[node].logic {
                        Logic::OR => val1 || val2,
                        Logic::AND => val1 && val2,
                        Logic::XOR => (val1 && !val2) || (!val1 && val2),
                        _ => panic!(),
                    };
                    g[node].value = Some(new_bool);
                } else {
                    queue.push_back(node);
                }
            }
            _ => panic!(),
        }
    }

    let mut z_nodes = map
        .keys()
        .filter(|k| k.starts_with('z'))
        .collect::<Vec<_>>();
    z_nodes.sort_unstable();
    //println!("{:?}", z_nodes);
    //
    // let z_vals = z_nodes
    //     .iter()
    //     .map(|&n| g[*map.get(n).unwrap()].value)
    //     .collect::<Vec<_>>();
    // println!("{:?}", z_vals);

    let mut final_output = 0;
    for &z_node in z_nodes.iter().rev() {
        let &index = map.get(z_node).unwrap();
        final_output = (final_output << 1) | g[index].value.unwrap() as usize
    }

    println!("{}", final_output);
}
