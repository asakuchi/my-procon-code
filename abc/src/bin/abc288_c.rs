use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);

    let mut result = 0;

    for &(a, b) in &a_b {
        if uf.equiv(a, b) {
            result += 1;
        } else {
            uf.union(a, b);
        }
    }

    println!("{}", result);
}
