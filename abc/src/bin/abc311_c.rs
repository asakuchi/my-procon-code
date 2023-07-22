use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut uf = UnionFind::new(n);

    for i in 0..n {
        if uf.equiv(i, a[i]) {
            // i から a[i] の向きで閉路ができる

            let mut result = Vec::new();

            result.push(i);

            let mut current = a[i];

            while current != i {
                result.push(current);
                current = a[current];
            }

            println!("{}", result.len());

            let text = result.into_iter().map(|x| x + 1).format(" ");
            println!("{}", text);

            return;
        }

        uf.union(i, a[i]);
    }
}
