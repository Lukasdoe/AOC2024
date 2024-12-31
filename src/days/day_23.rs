use crate::input::{Input, Part};
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};

pub(super) fn run(input: &Input, part: Part) -> String {
    let mut net = petgraph::graph::UnGraph::new_undirected();
    let mut nodes = HashMap::new();
    let mut nodes_rev = HashMap::new();
    for line in input.get().lines() {
        let (n1, n2) = line.split_once("-").unwrap();
        let n1 = match nodes.get(&n1) {
            Some(n) => *n,
            None => {
                let n = net.add_node(0);
                nodes.insert(n1, n);
                nodes_rev.insert(n, n1);
                n
            }
        };
        let n2 = match nodes.get(&n2) {
            Some(n) => *n,
            None => {
                let n = net.add_node(0);
                nodes.insert(n2, n);
                nodes_rev.insert(n, n2);
                n
            }
        };
        net.add_edge(n1, n2, 0);
    }
    if part == Part::One {
        let subtree_cnt = nodes
            .iter()
            .filter(|(k, _)| k.starts_with("t"))
            .flat_map(|(_, v)| {
                net.neighbors_undirected(*v)
                    .combinations(2)
                    .map(|c| (*v, c[0], c[1]))
                    .filter(|(n1, n2, n3)| {
                        net.contains_edge(*n1, *n2)
                            && net.contains_edge(*n1, *n3)
                            && net.contains_edge(*n2, *n3)
                    })
                    .map(|(n1, n2, n3)| {
                        (
                            nodes_rev[&n1].to_string(),
                            nodes_rev[&n2].to_string(),
                            nodes_rev[&n3].to_string(),
                        )
                    })
                    .map(|(n1, n2, n3)| {
                        let mut v = vec![n1, n2, n3];
                        v.sort();
                        v
                    })
            })
            .unique()
            .count();
        subtree_cnt.to_string()
    } else {
        let max_cliques = bron_kerbosch(&net);
        max_cliques
            .iter()
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .map(|n| nodes_rev[n].to_string())
            .sorted()
            .join(",")
    }
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
fn bron_kerbosch(g: &UnGraph<usize, u32>) -> Vec<HashSet<NodeIndex>> {
    fn bronkerbosch(
        p: HashSet<NodeIndex>,
        r: HashSet<NodeIndex>,
        x: HashSet<NodeIndex>,
        graph: &UnGraph<usize, u32>,
        max_cliques: &mut Vec<HashSet<NodeIndex>>,
    ) {
        let mut p_fp = p.clone();
        let mut x_fp = x.clone();

        if p.is_empty() {
            if x.is_empty() {
                max_cliques.push(r.clone());
            }
            return;
        }

        for v in p.iter() {
            let v_neighbours = graph.neighbors(*v).collect::<HashSet<NodeIndex>>();

            let p_intersect_v_neighbors = p_fp.intersection(&v_neighbours).cloned().collect();
            let mut r_union_v = r.clone();
            r_union_v.insert(*v);
            let x_intersect_v_neighbors = x_fp.intersection(&v_neighbours).cloned().collect();

            bronkerbosch(
                p_intersect_v_neighbors,
                r_union_v,
                x_intersect_v_neighbors,
                graph,
                max_cliques,
            );

            p_fp.remove(v);
            x_fp.insert(*v);
        }
    }
    let p = g.node_indices().collect();
    let r = HashSet::new();
    let x = HashSet::new();
    let mut max_cliques = Vec::new();
    bronkerbosch(p, r, x, g, &mut max_cliques);
    max_cliques
}
