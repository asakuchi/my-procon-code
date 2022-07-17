use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    if s[0] != s[1] && s[0] != s[2] {
        println!("{}", s[0]);
    } else if s[1] != s[0] && s[1] != s[2] {
        println!("{}", s[1]);
    } else if s[2] != s[0] && s[2] != s[0] {
        println!("{}", s[2]);
    } else {
        println!("-1");
    }
}
