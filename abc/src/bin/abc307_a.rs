use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 7],
    }

    let mut list = vec![0; n];

    for i in 0..n {
        for j in 0..7 {
            list[i] += a[i * 7 + j];
        }
    }

    let text = list.iter().join(" ");

    println!("{}", text);
}
