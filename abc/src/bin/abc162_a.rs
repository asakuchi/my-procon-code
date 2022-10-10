use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    for c in n {
        if c == '7' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
