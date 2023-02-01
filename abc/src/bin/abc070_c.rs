use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let mut result = 1;

    for x in t {
        result = result.lcm(&x);
    }

    println!("{}", result);
}
