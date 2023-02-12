use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;

    for mask in 0..1 << (s.len() - 1) {
        let mut x = vec![s[0]];

        for i in 1..s.len() {
            // + がある
            if mask & 1 << (i - 1) > 0 {
                let text = x.iter().format("").to_string();
                let addition: usize = text.parse().unwrap();
                result += addition;

                x = vec![];
            }

            x.push(s[i]);
        }

        if x.len() > 0 {
            let text = x.iter().format("").to_string();
            let addition: usize = text.parse().unwrap();
            result += addition;
        }
    }

    println!("{}", result);
}
