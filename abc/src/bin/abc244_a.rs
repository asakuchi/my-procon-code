use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    println!("{}", &s[n - 1..]);
}
