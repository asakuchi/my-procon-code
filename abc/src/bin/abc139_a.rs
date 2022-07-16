use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
use proconio::marker::Chars;
// use proconio::marker::Usize1;
// use itertools::izip;
// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut result = 0;

    for i in 0..3 {
        if s[i] == t[i] {
            result += 1;
        }
    }

    println!("{}", result);
}
