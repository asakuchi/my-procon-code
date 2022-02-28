//!
//! AtCoder では petgraph を使う
//!

// use proconio::fastout;
// use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

use petgraph::algo::min_spanning_tree;
use petgraph::data::Element;
use petgraph::prelude::*;
use petgraph::Graph;

///
/// 最小全域木
/// クラスカルのアルゴリズム
/// Kruskal's Algorithm
///
// #[fastout]
fn main() {
    // input! {
    //     v: usize, // 頂点数
    //     e: usize, // 辺の数
    //     mut s_t_w: [(usize, usize, usize); e], // 辺が結ぶ2点の頂点と重み
    // }

    let v = 6;
    let e = 9;

    let s_t_w = vec![
        (0, 1, 1),
        (0, 2, 3),
        (1, 2, 1),
        (1, 3, 7),
        (2, 4, 1),
        (1, 4, 3),
        (3, 4, 1),
        (3, 5, 1),
        (4, 5, 6),
    ];

    // ------------------------------------

    // Nodeの重み : ()
    // Edgeの重み : usize
    // Edge type : Undirected 無向グラフ
    // Index Type : usize
    let mut graph: Graph<(), usize, Undirected, usize> = Graph::with_capacity(v, e);

    graph.extend_with_edges(&s_t_w);
    // graph.add_edge(c.into(), d.into(), e);

    // 最小全域木
    let res = min_spanning_tree(&graph);

    // for value in res {
    //     match value {
    //         Element::Node { weight } => {
    //             println!("node {:?}", weight);
    //         }
    //         Element::Edge {
    //             source,
    //             target,
    //             weight,
    //         } => {
    //             println!("edge {:?} {:?} {:?}", source, target, weight);
    //         }
    //     }
    // }

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    for value in res {
        if let Element::Edge {
            source: _,
            target: _,
            weight,
        } = value
        {
            total_cost += weight;
        }
    }

    println!("{}", total_cost);
}
