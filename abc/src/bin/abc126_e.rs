use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x_y_z: [(Usize1, Usize1, usize); m],
    }

    let mut uf = UnionFind::new(n);

    for &(x, y, _) in &x_y_z {
        uf.union(x, y);
    }

    let mut head = HashSet::new();

    for i in 0..n {
        head.insert(uf.find(i));
    }

    println!("{}", head.len());
}
