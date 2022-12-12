use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s_c: [(Usize1, usize); m],
    }

    let mut text = vec![None; n];

    for &(s, c) in &s_c {
        if let Some(prev) = text[s] {
            if prev != c {
                println!("-1");
                return;
            }
        } else {
            text[s] = Some(c);
        }
    }

    if n == 1 {
        if let Some(value) = text[0] {
            println!("{}", value);
        } else {
            println!("0");
        }
        return;
    }

    let mut result = Vec::new();

    // 先頭1桁
    if let Some(value) = text[0] {
        if value == 0 {
            println!("-1");
            return;
        }

        result.push(value);
    } else {
        result.push(1);
    }

    for i in 1..n {
        if let Some(value) = text[i] {
            result.push(value);
        } else {
            result.push(0);
        }
    }

    let line = result.iter().format("");

    println!("{}", line);
}
