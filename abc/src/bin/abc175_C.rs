use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: i128,
        mut k: i128,
        d: i128,
    }

    if x.abs() > k * d {
        println!("{}", x.abs() - k * d);
        return;
    }

    let rest = x.abs() % d;
    k -= x.abs() / d;

    if k % 2 == 0 {
        println!("{}", rest);
    } else {
        println!("{}", (rest - d).abs());
    }
}
