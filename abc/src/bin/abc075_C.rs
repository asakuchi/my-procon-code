use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

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
