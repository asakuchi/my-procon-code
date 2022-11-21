use std::collections::HashSet;

use petgraph::algo::min_spanning_tree;
use petgraph::data::Element;
use petgraph::prelude::*;
use petgraph::Directed;
use petgraph::Graph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_c: [(Usize1, Usize1, isize); m],
    }

    // Nodeの重み : ()
    // Edgeの重み : usize
    // Edge type : Undirected 無向グラフ
    // Index Type : usize
    let mut graph: Graph<(), isize, Directed, usize> = Graph::with_capacity(n, m);

    graph.extend_with_edges(&a_b_c);

    // 最小全域木
    let res = min_spanning_tree(&graph);

    let mut mst_edge = HashSet::new();

    for value in res {
        if let Element::Edge {
            source,
            target,
            weight,
        } = value
        {
            mst_edge.insert((source, target, weight));
        }
    }

    let mut result = 0;

    for &(a, b, c) in &a_b_c {
        if mst_edge.contains(&(a, b, c)) {
            continue;
        }

        // 最小全域木に含まれない木なら削除して報酬を得ても良い
        if c >= 0 {
            result += c;
        }
    }

    println!("{}", result);
}
