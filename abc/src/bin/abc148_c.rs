use num::Integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a.lcm(&b));
}
