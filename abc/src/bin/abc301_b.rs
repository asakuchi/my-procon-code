use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = Vec::new();

    result.push(a[0]);

    for i in 1..n {
        if a[i - 1] > a[i] {
            for j in (a[i]..a[i - 1]).rev() {
                result.push(j);
            }
        } else {
            for j in a[i - 1] + 1..=a[i] {
                result.push(j);
            }
        }
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
