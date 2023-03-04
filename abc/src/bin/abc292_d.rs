use std::collections::{HashMap, HashSet};

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut map = HashMap::new();

    let mut uf = UnionFind::new(n);

    for &(u, v) in &u_v {
        uf.union(u, v);
    }

    for &(u, v) in &u_v {
        let count = map.entry(uf.find(u)).or_insert((HashSet::new(), 0));

        count.0.insert(u);
        count.0.insert(v);
        count.1 += 1;
    }

    for i in 0..n {
        if !map.contains_key(&uf.find(i)) {
            // 頂点数1 の辺なし
            println!("No");
            return;
        }
    }

    for (_v, (set, e_count)) in &map {
        if set.len() != *e_count {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
