use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [usize; n],
    }

    let mut uf = UnionFind::new(n);

    let mut result = 0;

    for i in 0..n {
        if !uf.equiv(i, x[i]) {
            uf.union(i, x[i]);
            continue;
        }

        let mut cur = c[i];
        let mut v = i;

        v = x[v];
        cur = cur.min(c[v]);

        while v != i {
            v = x[v];
            cur = cur.min(c[v]);
        }

        result += cur;
    }

    println!("{}", result);
}
