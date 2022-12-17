use std::cmp::Reverse;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut s_t_w = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            s_t_w.push((i, j, (mod_pow(a[i], a[j], m) + mod_pow(a[j], a[i], m)) % m));
        }
    }

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    // 重みの昇順にソート
    s_t_w.sort_by_key(|x| Reverse(x.2));

    let mut uf = UnionFind::new(n);

    for i in 0..s_t_w.len() {
        if !uf.equiv(s_t_w[i].0, s_t_w[i].1) {
            total_cost += s_t_w[i].2;
            uf.union(s_t_w[i].0, s_t_w[i].1);
        }
    }

    println!("{}", total_cost);
}

fn mod_pow(x: usize, a: usize, m: usize) -> usize {
    if a == 1 {
        return x;
    }

    if a % 2 == 1 {
        return (x * mod_pow(x, a - 1, m)) % m;
    }

    let t = mod_pow(x, a / 2, m);

    return (t * t) % m;
}
