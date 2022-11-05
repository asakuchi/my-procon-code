use itertools::Itertools;
use proconio::fastout;
use proconio::{input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    for i in 0..n {
        list[i].sort();

        let text = list[i].iter().map(|&x| x + 1).join(" ");

        println!("{} {}", list[i].len(), text);
    }
}
