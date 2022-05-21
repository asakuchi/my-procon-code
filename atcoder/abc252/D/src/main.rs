use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut miman = vec![0; 2_000_001];

    for i in 0..n {
        miman[a[i]] += 1;
    }

    for i in 0..2_000_000 {
        miman[i + 1] += miman[i];
    }

    let mut result = 0;

    for j in 0..n {
        result += miman[a[j] - 1] * (n - miman[a[j]]);
    }

    println!("{}", result);
}
