use proconio::fastout;
use proconio::input;
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();

    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for target in x {
        let mut result = 0;

        let index_low = a.lower_bound(&target);

        // 不足
        result += (target * index_low) - s[index_low];

        let index_high = a.lower_bound(&(target + 1));

        // 超過
        result += s[s.len() - 1] - s[index_high] - (target * (s.len() - 1 - index_high));

        println!("{}", result);
    }
}
