use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for k in 1..=n {
        let mut max_value = 0;

        for i in k..n + 1 {
            let value = s[i] - s[i - k];

            max_value = std::cmp::max(max_value, value);
        }

        println!("{}", max_value);
    }
}
