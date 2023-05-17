use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut x = vec![0; n];

    // 仮で x_0 を 0 とする

    for i in 1..n {
        x[i] = a[i - 1] * 2 - x[i - 1];
    }

    let diff = (a[n - 1] * 2 - x[n - 1]) / 2;

    for i in 0..n {
        if i % 2 == 0 {
            x[i] += diff;
        } else {
            x[i] -= diff;
        }
    }

    let text = x.iter().format(" ");

    println!("{}", text);
}
