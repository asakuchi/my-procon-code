use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }

    let ng: Vec<_> = a.iter().filter(|&x| *x < p).collect();

    println!("{}", ng.len());
}
