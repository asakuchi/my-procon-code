use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut has_sweet = vec![false; n];

    for _ in 0..k {
        input! {
            d: usize,
            a: [Usize1; d],
        }

        for snuke in a {
            has_sweet[snuke] = true;
        }
    }

    let count = has_sweet.iter().filter(|&&x| !x).count();

    println!("{}", count);
}
