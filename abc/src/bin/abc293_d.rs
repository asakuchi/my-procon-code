use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        rope: [(Usize1, char ,Usize1, char); m],
    }

    let mut has_roop = 0;

    let mut uf = UnionFind::new(n);

    for &(a, _b, c, _d) in &rope {
        if uf.equiv(a, c) {
            has_roop += 1;
        } else {
            uf.union(a, c);
        }
    }

    let mut components = 0;

    for i in 0..n {
        if uf.find(i) == i {
            components += 1;
        }
    }

    println!("{} {}", has_roop, components - has_roop);
}
