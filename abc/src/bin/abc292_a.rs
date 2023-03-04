use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for c in s {
        print!("{}", c.to_uppercase());
    }

    println!();
}
