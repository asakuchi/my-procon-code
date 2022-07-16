use proconio::fastout;
use proconio::input;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut list = vec![0; n + 1];

    list[n] = 1;

    for i in (0..n).rev() {
        if i + 1 <= n {
            list[i] += list[i + 1];
            list[i] %= MODULO;
        }
        if i + l <= n {
            list[i] += list[i + l];
            list[i] %= MODULO;
        }
    }

    println!("{}", list[0]);
}
