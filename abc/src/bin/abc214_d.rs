use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut u_v_w: [(Usize1, Usize1, usize); n - 1],
    }

    u_v_w.sort_by_key(|e| e.2);

    let mut uf = UnionFind::new(n);

    let mut size = vec![1; n];

    let mut result = 0;

    for &(u, v, w) in &u_v_w {
        let u_size = size[uf.find(u)];
        let v_size = size[uf.find(v)];

        result += w * u_size * v_size;

        uf.union(u, v);
        size[uf.find(u)] = u_size + v_size;
    }

    println!("{}", result);
}
