use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", if s.contains(&'9') { "Yes" } else { "No" });
}
