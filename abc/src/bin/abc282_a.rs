use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let result = ('A'..='Z').take(k).join("");

    println!("{}", result);
}
