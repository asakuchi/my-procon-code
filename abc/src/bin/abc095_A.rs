use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", 700 + s.iter().filter(|&&c| c == 'o').count() * 100);
}
