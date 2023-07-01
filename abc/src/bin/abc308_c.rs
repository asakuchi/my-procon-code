use itertools::Itertools;
use num::rational::Ratio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut list = a_b
        .iter()
        .enumerate()
        .map(|(i, &(a, b))| (Ratio::new(a, a + b), i))
        .collect::<Vec<_>>();

    list.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    let text = list.iter().map(|(_, i)| i + 1).join(" ");

    println!("{}", text);
}
