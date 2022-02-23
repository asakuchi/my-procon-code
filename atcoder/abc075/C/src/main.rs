use proconio::fastout;
use proconio::input;

use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab_org: [(usize, usize); m],
    }

    let ab: Vec<_> = ab_org.into_iter().map(|(a, b)| (a - 1, b - 1)).collect();

    let mut result = 0;

    for i in 0..m {
        let mut set = UnionFind::new(n);

        for (j, &(a, b)) in ab.iter().enumerate() {
            if i == j {
                continue;
            }

            set.union(a, b);
        }

        if !set.equiv(ab[i].0, ab[i].1) {
            result += 1;
        }
    }

    println!("{}", result);
}
