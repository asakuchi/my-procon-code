use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n * 2 >= m {
        println!("{}", m / 2);
        return;
    }

    let rest = m - n * 2;

    println!("{}", n + rest / 4);
}
