use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; 2]; 2],
    }

    println!("{}", a[r - 1][c - 1]);
}
