use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    for i in 0..=n {
        for j in 0..=m {
            if j * n + i * m - i * j * 2 == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
