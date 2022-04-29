use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    // 入力値
    let v = n + 1; // 頂点の数
    let e = 2 * n; // 辺の数
    let r = 0; // 始点
    let mut s_t_d = Vec::new();

    // 高橋君からすぬけ君
    for i in 0..n {
        s_t_d.push((0, i + 1, t[i]));
    }

    // i番目のすぬけ君からi+1番目のすぬけ君
    for i in 0..n - 1 {
        s_t_d.push((i + 1, i + 2, s[i]));
    }

    s_t_d.push((n, 1, s[n - 1]));

    /* --------------------------------------------------- */

    // Nodeの重み : ()
    // Edgeの重み : usize
    // Edge type : Directed 有向グラフ
    // Index Type : usize
    let mut graph: Graph<(), usize, Directed, usize> = Graph::with_capacity(v, e);

    // graph.add_edge(c.into(), d.into(), e);
    graph.extend_with_edges(&s_t_d);

    let res = dijkstra(&graph, r.into(), None, |e| *e.weight());

    for i in 1..v {
        match res.get(&i.into()) {
            Some(value) => println!("{}", value),
            None => println!("INF"),
        }
    }
}
