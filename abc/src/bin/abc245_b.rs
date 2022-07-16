use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in 0..=2001 {
        if !a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
