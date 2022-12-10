use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    if n != 8 {
        println!("No");
        return;
    }

    if s[0] < 'A' || s[0] > 'Z' || s[n - 1] < 'A' || s[n - 1] > 'Z' {
        println!("No");
        return;
    }

    let text = &s[1..n - 1].iter().join("");

    if let Ok(x) = text.parse::<usize>() {
        if x >= 100000 && x <= 999999 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
