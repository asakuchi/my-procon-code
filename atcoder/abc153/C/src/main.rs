use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    if k >= n {
        println!("{}", 0);
        return;
    }

    h.sort();

    // println!("{:?}", h);

    let mut count: u128 = 0;

    for i in 0..n - k {
        count += h[i] as u128;
    }

    println!("{}", count);
}
