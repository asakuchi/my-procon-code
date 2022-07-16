use num::Integer;
use proconio::input;

const MAX_VALUE: u128 = 1000000000000000000;

fn main() {
    input! {
        a: u128,
        b: u128,
    }

    let lcm = a.lcm(&b);
    if lcm > MAX_VALUE {
        println!("Large");
    } else {
        println!("{}", lcm);
    }
}
