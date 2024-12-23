#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use petgraph::graph::UnGraph;
use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};
use petgraph::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    const PATH: &str = "day23/src/day23_input.txt";
    part1(PATH);
    part2(PATH);
}

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let g = build_graph(&input);
    let t_nodes = g.node_indices().filter(|&n| g[n].starts_with('t'));

    let mut set = HashSet::new();
    for v in t_nodes {
        for n in g.neighbors(v) {
            for n2 in g.neighbors(n) {
                for n3 in g.neighbors(n2) {
                    if n3 == v {
                        let mut network = vec![g[v], g[n], g[n2]];
                        network.sort_unstable();
                        set.insert(network);
                    }
                }
            }
        }
    }
    println!("{}", set.len());
}

fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let g = build_graph(&input);

    let mut cliques = maximal_cliques(&g);
    cliques.sort_by_key(HashSet::len);
    let mut max = cliques
        .last()
        .unwrap()
        .iter()
        .map(|&n| g[n])
        .collect::<Vec<_>>();
    max.sort_unstable();

    println!("{:?}", max.join(","));
}

fn build_graph(input: &str) -> UnGraph<&str, ()> {
    let connections = input
        .lines()
        .map(|s| (&s[0..2], &s[3..]))
        .collect::<Vec<_>>();

    let mut nodes = HashMap::new();

    let mut g: Graph<&str, (), _> = UnGraph::new_undirected();
    for (c1, c2) in connections {
        let c1_node = if let Some(node) = nodes.get(&c1) {
            *node
        } else {
            let index = g.add_node(c1);
            nodes.insert(c1, index);
            index
        };

        let c2_node = if let Some(node) = nodes.get(&c2) {
            *node
        } else {
            let index = g.add_node(c2);
            nodes.insert(c2, index);
            index
        };

        g.add_edge(c1_node, c2_node, ());
    }

    g
}

pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::new();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::new();
    bron_kerbosch_pivot(g, &adj_mat, r, p, x)
}

fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .copied()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .copied()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .copied()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}
