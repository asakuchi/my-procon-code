use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let text = "atcoder";

    println!("{}", &text[l - 1..r]);
}
