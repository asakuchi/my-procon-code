use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    let mut result = 0;

    let mut prev = s[0];

    for &c in &s {
        if prev == c {
            // do noting.
        } else {
            prev = c;
            result += 1;
        }
    }

    println!("{}", result);
}
