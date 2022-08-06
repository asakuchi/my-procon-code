use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    'perm: for permutation in (1..=m).permutations(n) {
        let mut prev = 0;
        let mut list = Vec::new();

        for value in permutation {
            if prev > value {
                continue 'perm;
            }

            list.push(value);
            prev = value;
        }

        let text = list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", text);
    }
}
