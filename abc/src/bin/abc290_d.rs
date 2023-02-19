use num_integer::gcd;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        d: usize,
        k: Usize1,
    }

    let cycle = n / gcd(d, n);

    // (k % cycle) * d % n : 1サイクルでの位置
    // (k / cycle) : サイクルが進むごとに1ずらす
    let pos = (k % cycle) * d % n + (k / cycle);

    println!("{}", pos);
}
