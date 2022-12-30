use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b_1 = a[1];

    for i in 2..n {
        b_1 ^= a[i];
    }

    let all_a = b_1 ^ a[0];

    let mut result = Vec::new();

    for i in 0..n {
        result.push(all_a ^ a[i]);
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
