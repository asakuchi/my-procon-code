use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s = 0;

    for i in 0..n {
        s = s + a[i] + s;
    }

    println!("{}", s);
}
