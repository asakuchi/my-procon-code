use num::BigUint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a_b: [(usize, usize); n],
    }

    let mut s = BigUint::from(1usize) << x.into();

    // println!("{:b}", s);

    for i in 0..n {
        let (a, b) = a_b[i];

        // println!("use {}", a);

        for _ in 0..b {
            s |= s.clone() >> a;

            // println!("{:b}", s);
        }
    }

    if s & BigUint::from(1usize) > BigUint::from(0usize) {
        println!("Yes");
    } else {
        println!("No");
    }
}
