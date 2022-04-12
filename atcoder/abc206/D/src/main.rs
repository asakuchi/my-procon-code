use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    let mut set = UnionFind::new(3_00_000);

    for i in 0..n / 2 {
        let left = a[i];
        let right = a[n - 1 - i];

        if !set.equiv(left, right) {
            result += 1;
            set.union(left, right);
        }
    }

    println!("{}", result);
}
