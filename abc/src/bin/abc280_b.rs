use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [isize; n],
    }

    let mut a = vec![0; n];

    a[0] = s[0];

    for i in 1..n {
        a[i] = s[i] - s[i - 1];
    }

    let result = a.iter().format(" ");

    println!("{}", result);
}
