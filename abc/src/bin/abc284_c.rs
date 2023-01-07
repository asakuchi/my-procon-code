use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);

    for (u, v) in u_v {
        uf.union(u, v);
    }

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(uf.find(i));
    }

    println!("{}", set.len());
}
