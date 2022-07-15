use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 0..n {
        if s[i] == '1' {
            if i % 2 == 0 {
                println!("Takahashi");
                return;
            } else {
                println!("Aoki");
                return;
            }
        }
    }
}
