use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }

    if n > a {
        println!("{}", x * a + y * (n - a));
    } else {
        println!("{}", x * n);
    }
}
