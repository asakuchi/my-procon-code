use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }

    println!("{}", (32 as usize).pow(a - b));
}
