use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let result = a
        .iter()
        .enumerate()
        .sorted_by(|&a, &b| a.1.cmp(b.1).reverse())
        .map(|x| x.0 + 1)
        .format(" ");

    println!("{}", result);
}
