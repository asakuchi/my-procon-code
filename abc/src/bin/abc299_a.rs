use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 0..n {
        if s[i] == '*' {
            if s[..i].contains(&'|') && s[i..].contains(&'|') {
                println!("in");
                return;
            }
        }
    }

    println!("out");
}
