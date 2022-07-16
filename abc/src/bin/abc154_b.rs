use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("{}", "x".repeat(s.len()));
}
