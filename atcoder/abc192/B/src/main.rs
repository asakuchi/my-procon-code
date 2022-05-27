use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for (i, &c) in s.iter().enumerate() {
        if i % 2 == 0 && 'A' <= c && c <= 'Z' {
            println!("No");
            return;
        } else if i % 2 != 0 && 'a' <= c && c <= 'z' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
