use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut list = Vec::new();

    for c in 'A' as u8..='Z' as u8 {
        for _ in 0..n {
            list.push(c);
        }
    }

    println!("{}", list[x - 1] as char);
}
