use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut rest = k;

    let mut result = Vec::new();

    for i in 0..n {
        if s[i] == 'o' && rest > 0 {
            result.push('o');
            rest -= 1;
        } else {
            result.push('x');
        }
    }

    let text = result.iter().format("");

    println!("{}", text);
}
