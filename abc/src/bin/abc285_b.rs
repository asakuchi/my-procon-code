use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..=n - 1 {
        let mut count = 0;

        for k in 0..n - i {
            if s[k] != s[k + i] {
                count += 1;
            } else {
                break;
            }
        }
        println!("{}", count);
    }
}
