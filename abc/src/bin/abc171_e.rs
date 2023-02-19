use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut all = 0;

    for i in 0..n {
        all ^= a[i];
    }

    let mut result = Vec::new();

    for &x in &a {
        result.push(all ^ x);
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
