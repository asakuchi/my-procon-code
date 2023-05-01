use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if s[..n / 2] == s[n / 2..] {
        println!("Yes");
    } else {
        println!("No");
    }
}
