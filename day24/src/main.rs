#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::{Directed, Direction, Graph};
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::sync::LazyLock;

static WIRE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([a-z0-9]{3}): ([01])").unwrap());
static GATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap()
});

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Logic {
    Or,
    And,
    Xor,
    Unknown,
    None,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GateNode {
    name: String,
    logic: Logic,
    value: Option<bool>,
}

fn main() {
    const PATH: &str = "day24/src/day24_input.txt";
    //part1(PATH);
    part2(PATH);
}

fn part1(path: &str) {
    //-> (Graph<GateNode, ()>, HashMap<&str, NodeIndex>) {
    let input = fs::read_to_string(path).unwrap();
    let (mut g, map) = build_graph(&input);

    execute_circuit(&mut g);

    let mut z_nodes = map
        .keys()
        .filter(|k| k.starts_with('z'))
        .collect::<Vec<_>>();
    z_nodes.sort_unstable();

    let final_output = z_nodes.iter().rev().fold(0, |acc, &index| {
        (acc << 1) | g[*map.get(index).unwrap()].value.unwrap() as usize
    });
    println!("{final_output}");

    let viz_output = Dot::with_config(&g, &[Config::EdgeNoLabel]);
    fs::write("day24/src/viz.dot", format!("{viz_output:?}")).unwrap();
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let (mut g, map) = build_graph(&input);
    execute_circuit(&mut g);

    let x_indices = sorted_nodes_start_with('x', &map);
    let y_indices = sorted_nodes_start_with('y', &map);
    let z_indices = sorted_nodes_start_with('z', &map);

    assert_eq!(x_indices.len(), y_indices.len());
    assert_eq!(x_indices.len() + 1, z_indices.len());

    for i in 1..x_indices.len() {
        check_circuit(&g, x_indices[i], y_indices[i]);
    }
}

fn check_circuit(g: &Graph<GateNode, ()>, x_index: NodeIndex, y_index: NodeIndex) {
    let x_neighbors = g.neighbors(x_index).collect::<Vec<_>>();
    let y_neighbors = g.neighbors(y_index).collect::<Vec<_>>();
    assert_eq!(x_neighbors.len(), 2);
    assert_eq!(y_neighbors.len(), 2);

    // Confirmed that all x## and y## lead to the same AND and XOR
    if x_neighbors[0] == y_neighbors[0] {
        if x_neighbors[1] != y_neighbors[1] {
            println!("Error with {:?} {:?}", g[x_index], g[y_index]);
        }
        //println!("{:?} {:?} OK!", g[x_index], g[y_index]);
    } else if x_neighbors[0] == y_neighbors[1] {
        if x_neighbors[1] != y_neighbors[0] {
            println!("Error with {:?} {:?}", g[x_index], g[y_index]);
        }
        //println!("{:?} {:?} OK!", g[x_index], g[y_index]);
    } else {
        println!("Error with {:?} {:?}", g[x_index], g[y_index]);
    }

    let xor1_index = if g[x_neighbors[0]].logic == Logic::Xor {
        x_neighbors[0]
    } else {
        x_neighbors[1]
    };

    let xor1_neighbors = g.neighbors(xor1_index).collect::<Vec<_>>();
    if xor1_neighbors.len() != 2 {
        println!("Error at {:?}", g[xor1_index]);
    } else {
        let xor2_index = if g[xor1_neighbors[0]].logic == Logic::Xor {
            xor1_neighbors[0]
        } else {
            xor1_neighbors[1]
        };

        if !g[xor2_index].name.starts_with('z') {
            println!(
                "Error with {:?}. Should be z{}",
                g[xor2_index],
                g[x_index].name[1..].to_owned()
            );
        }
    }
}

fn sorted_nodes_start_with(c: char, map: &HashMap<&str, NodeIndex>) -> Vec<NodeIndex> {
    let mut nodes = map.keys().filter(|k| k.starts_with(c)).collect::<Vec<_>>();
    nodes.sort_unstable();
    nodes
        .iter()
        .map(|&n| *map.get(n).unwrap())
        .collect::<Vec<_>>()
}

fn execute_circuit(g: &mut Graph<GateNode, (), Directed>) {
    let mut queue = g.node_indices().collect::<VecDeque<_>>();
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let inputs = g
            .neighbors_directed(node, Direction::Incoming)
            .collect::<Vec<_>>();

        match inputs.len() {
            0 => continue,
            2 => {
                if let (Some(val1), Some(val2)) = (g[inputs[0]].value, g[inputs[1]].value) {
                    let new_bool = match g[node].logic {
                        Logic::Or => val1 || val2,
                        Logic::And => val1 && val2,
                        Logic::Xor => (val1 && !val2) || (!val1 && val2),
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
}

fn build_graph(input: &str) -> (Graph<GateNode, ()>, HashMap<&str, NodeIndex>) {
    let mut map = HashMap::new();
    let mut g = DiGraph::new();

    // Load initial nodes
    for c in WIRE_RE.captures_iter(input) {
        let node = c.get(1).unwrap().as_str();
        let val = c.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let new_node = GateNode {
            name: node.to_string(),
            logic: Logic::None,
            value: Some(val != 0),
        };
        let index = g.add_node(new_node);
        map.insert(node, index);
    }

    // Load all gate nodes
    for c in GATE_RE.captures_iter(input) {
        let input_nodes = [c.get(1).unwrap().as_str(), c.get(3).unwrap().as_str()];
        let input_node_indices = input_nodes.map(|n| {
            if let Some(&node) = map.get(n) {
                node
            } else {
                let new_node = GateNode {
                    name: n.to_string(),
                    logic: Logic::Unknown,
                    value: None,
                };
                let index = g.add_node(new_node);
                map.insert(n, index);
                index
            }
        });

        let output_node_str = c.get(4).unwrap().as_str();
        let logic = match c.get(2).unwrap().as_str() {
            "OR" => Logic::Or,
            "AND" => Logic::And,
            "XOR" => Logic::Xor,
            _ => panic!(),
        };
        let output_node_index = if let Some(&node) = map.get(output_node_str) {
            g[node].logic = logic;
            node
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

    (g, map)
}
