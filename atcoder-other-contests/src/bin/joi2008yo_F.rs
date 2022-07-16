use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut graph: Graph<(), usize, Undirected, usize> = Graph::with_capacity(n, n);

    for _ in 0..n {
        graph.add_node(());
    }

    for _ in 0..k {
        input! {record_type:usize}

        match record_type {
            // 注文票
            0 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }

                // aからbまでの最短コスト
                let res = dijkstra(&graph, a.into(), Some(b.into()), |e| *e.weight());

                match res.get(&b.into()) {
                    Some(value) => println!("{}", value),
                    None => println!("-1"),
                }
            }
            // 運行情報
            _ => {
                input! {
                    c: Usize1,
                    d: Usize1,
                    e: usize,
                }

                graph.add_edge(c.into(), d.into(), e);
            }
        }
    }
}
