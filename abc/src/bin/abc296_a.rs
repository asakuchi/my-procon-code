use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let first = s[0];

    for i in 0..n {
        if i % 2 == 0 {
            if first != s[i] {
                println!("No");
                return;
            }
        } else {
            if first == s[i] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
