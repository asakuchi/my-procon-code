use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_c: [(Usize1, Usize1, usize); m],
    }

    /* --------------------------------------------------- */

    // Nodeの重み : ()
    // Edgeの重み : usize
    // Edge type : Undirected 無向グラフ
    // Index Type : usize
    let mut graph: Graph<(), usize, Undirected, usize> = Graph::with_capacity(n, m);

    // graph.add_edge(c.into(), d.into(), e);
    graph.extend_with_edges(&a_b_c);

    let res_from_0 = dijkstra(&graph, 0.into(), None, |e| *e.weight());
    let res_from_n = dijkstra(&graph, (n - 1).into(), None, |e| *e.weight());

    for i in 0..n {
        let to_i = res_from_0.get(&i.into()).unwrap();
        let from_i = res_from_n.get(&i.into()).unwrap();

        println!("{}", to_i + from_i);
    }
}
