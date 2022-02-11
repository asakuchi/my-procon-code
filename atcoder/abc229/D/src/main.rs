use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    let mut sum = vec![0; s.len() + 1];
    sum[0] = 0;

    for i in 0..s.len() {
        sum[i + 1] = sum[i];

        if s[i] == '.' {
            sum[i + 1] += 1;
        }
    }

    // println!("{:?}", sum);

    let mut s = 0;
    let mut t = 1;

    let mut result = 0;

    loop {
        // println!("pre {} {}", s, t);
        while t != sum.len() && sum[t] - sum[s] <= k {
            t += 1;
        }
        // println!("post {} {}", s, t);
        result = std::cmp::max(result, t - s - 1);

        if t == sum.len() {
            break;
        }

        s += 1;
    }

    println!("{}", result);
}
