use std::collections::HashMap;

use ac_library_rs::ModInt;
use proconio::input;

fn main() {
    input! {
        a: usize,
        x: usize,
        m: u32,
    }

    let mut memo = HashMap::new();

    ModInt::set_modulus(m);

    println!("{}", rec(a, x, &mut memo));
}

fn rec(a: usize, x: usize, memo: &mut HashMap<usize, ModInt>) -> ModInt {
    if x == 1 {
        return 1.into();
    }

    if let Some(&value) = memo.get(&x) {
        return value;
    }

    let temp = rec(a, x / 2, memo) + rec(a, x / 2, memo) * ModInt::new(a).pow(x as u64 / 2);

    let result = if x % 2 == 1 {
        temp * ModInt::new(a) + ModInt::new(1)
    } else {
        temp
    };

    memo.insert(x, result);

    // println!("x {} result {}", x, result);

    result
}
