use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut taka = 0;
    let mut aoki = 0;

    for i in 0..n {
        if s[i] == 'T' {
            taka += 1;
        } else {
            aoki += 1;
        }

        if taka >= (n + 1) / 2 {
            println!("T");
            return;
        }

        if aoki >= (n + 1) / 2 {
            println!("A");
            return;
        }
    }
}
