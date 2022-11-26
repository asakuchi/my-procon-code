use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;

    for c in s {
        if c == 'v' {
            result += 1;
        } else {
            result += 2;
        }
    }

    println!("{}", result);
}
