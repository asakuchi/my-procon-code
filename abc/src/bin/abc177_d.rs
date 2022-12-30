use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);

    let mut size = vec![1; n];

    for &(a, b) in &a_b {
        if !uf.equiv(a, b) {
            let size_a = size[uf.find(a)];
            let size_b = size[uf.find(b)];

            uf.union(a, b);

            size[uf.find(a)] = size_a + size_b;
        }
    }

    let mut max_size = 1;

    for i in 0..n {
        max_size = max_size.max(size[uf.find(i)]);
    }

    println!("{}", max_size);
}
