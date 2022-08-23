use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }

    let mut i = n;

    let mut result = Vec::new();

    while i != 0 {
        let mut updated = false;

        for j in (1..=m.min(i)).rev() {
            if s[i - j] == '1' {
                continue;
            }

            result.push(j);

            i -= j;
            updated = true;
            break;
        }

        if !updated {
            println!("-1");
            return;
        }
    }

    result.reverse();

    let text = result.iter().format(" ");

    println!("{}", text);
}
