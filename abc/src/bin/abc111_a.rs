use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    for c in n {
        if c == '1' {
            print!("9");
        } else if c == '9' {
            print!("1");
        }
    }

    println!();
}
