use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().filter(|&&value| value != x).format(" "));
}
