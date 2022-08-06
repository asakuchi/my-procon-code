use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for combination in (1..=m).combinations(n) {
        println!("{}", combination.iter().format(" "));
    }
}
