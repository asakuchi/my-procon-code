use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..s.len() {
        if s[i] >= 'A' && s[i] <= 'Z' {
            println!("{}", i + 1);
            return;
        }
    }
}
