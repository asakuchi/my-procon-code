use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    s.sort();

    let result: String = s.iter().collect();

    println!("{}", result);
}
