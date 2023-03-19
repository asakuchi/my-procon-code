use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = Vec::new();

    for i in 0..n {
        if a[i] % 2 == 0 {
            result.push(a[i]);
        }
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
