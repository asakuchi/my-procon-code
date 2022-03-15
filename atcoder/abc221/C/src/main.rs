use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut result = 0;

    for permutation in n.iter().permutations(n.len()) {
        for i in 1..n.len() {
            let mut left = Vec::new();
            let mut right = Vec::new();

            for j in 0..i {
                left.push(permutation[j]);
            }

            for j in i..n.len() {
                right.push(permutation[j]);
            }

            let left: usize = left.iter().map(|&c| c).collect::<String>().parse().unwrap();
            let right: usize = right
                .iter()
                .map(|&c| c)
                .collect::<String>()
                .parse()
                .unwrap();

            result = result.max(left * right);
        }
    }

    println!("{}", result);
}
