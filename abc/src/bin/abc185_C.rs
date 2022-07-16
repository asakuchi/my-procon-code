use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: usize,
    }

    let n = l - 1;

    println!("{}", combination(n, 11));
}

fn combination(n: usize, k: usize) -> usize {
    let mut result = 1;

    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}
