//!
//! 平方分割
//!
//! 例.
//! X=9として以下を求めるとする
//! 1 + A^1 + A^2 + A^3 + A^4 + A^5 + A^6 + A^7 + A^8
//!
//! 3分割する
//! (1 + A^1 + A^2) + (A^3 + A^4 + A^5) + (A^6 + A^7 + A^8)
//!
//! B = (1 + A^1 + A^2) とすると
//! 求めたいものは
//! B + B * A^3 + B * A^6
//!

use ac_library_rs::ModInt;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    }

    ModInt::set_modulus(m as u32);

    let sqrt_x = x.sqrt();

    let mut b = ModInt::new(0);

    let mut num = ModInt::new(1);

    for _ in 0..sqrt_x {
        b += num;

        num *= ModInt::new(a);
    }

    let mut c = ModInt::new(0);

    let a_pow_sqrt_x = ModInt::new(a).pow(sqrt_x as u64);

    for _ in 0..sqrt_x {
        c += b;

        b *= a_pow_sqrt_x;
    }

    for _ in 0..x - sqrt_x * sqrt_x {
        c *= a;
        c += ModInt::new(1);
    }

    println!("{}", c);
}
