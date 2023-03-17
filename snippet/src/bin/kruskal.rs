//!
//! 最小全域木
//!
//! クラスカルのアルゴリズム
//! Kruskal's Algorithm
//!
//! AOJ では自作コードを使う
//!

use petgraph::unionfind::UnionFind;

fn main() {
    let n = 6;
    let _m = 9;

    let mut u_v_w: Vec<(usize, usize, usize)> = vec![
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

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    // 重みの昇順にソート
    u_v_w.sort_by_key(|&(_, _, w)| w);

    let mut uf = UnionFind::new(n);

    for &(u, v, w) in &u_v_w {
        if !uf.equiv(u, v) {
            total_cost += w;
            uf.union(u, v);
        }
    }

    println!("{}", total_cost);
}
