use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use proconio::marker::Usize1;
// use itertools::izip;
// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
    }

    println!("{} {}", a.min(b), ((a + b) - n).max(0));
}
