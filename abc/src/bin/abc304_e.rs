use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
        k: usize,
        x_y: [(Usize1, Usize1); k],
        q: usize,
        p_q: [(Usize1, Usize1); q],
    }

    let mut uf = UnionFind::new(n);

    for &(u, v) in &u_v {
        uf.union(u, v);
    }

    let mut set = HashSet::new();

    for &(x, y) in &x_y {
        let root_x = uf.find(x);
        let root_y = uf.find(y);

        set.insert((root_x, root_y));
        set.insert((root_y, root_x));
    }

    for &(p, q) in &p_q {
        let root_p = uf.find(p);
        let root_q = uf.find(q);

        if !set.contains(&(root_p, root_q)) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
