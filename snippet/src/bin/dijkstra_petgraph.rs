//!
//! AtCoder では petgraph を使う
//!

use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
// use proconio::fastout;
// use proconio::input;

///
/// 単一始点最短経路
/// ダイクストラ
///
fn main() {
    // 入力値
    let v = 4; // 頂点の数
    let e = 5; // 辺の数
    let r = 0; // 始点
    let s_t_d = vec![(0, 1, 1), (0, 2, 4), (1, 2, 2), (2, 3, 1), (1, 3, 5)];

    /* --------------------------------------------------- */

    // Nodeの重み : ()
    // Edgeの重み : usize
    // Edge type : Undirected 無向グラフ
    // Index Type : usize
    let mut graph: Graph<(), usize, Undirected, usize> = Graph::with_capacity(v, e);

    // graph.add_edge(c.into(), d.into(), e);
    graph.extend_with_edges(&s_t_d);

    let res = dijkstra(&graph, r.into(), None, |e| *e.weight());

    for i in 0..v {
        match res.get(&i.into()) {
            Some(value) => println!("{}", value),
            None => println!("INF"),
        }
    }
}
