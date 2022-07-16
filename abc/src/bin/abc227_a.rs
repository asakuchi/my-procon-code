use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    }

    let mut current = a - 1;

    for _ in 0..k {
        current += 1;

        if current > n {
            current = 1;
        }
    }

    println!("{}", current);
}
