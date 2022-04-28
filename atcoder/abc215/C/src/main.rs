use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut list: Vec<_> = s.iter().permutations(s.len()).collect();

    list.sort();
    list.dedup();

    let result: String = list[k - 1].iter().join("");

    println!("{}", result);
}
