use std::collections::HashMap;

use proconio::input;

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
    }

    let mut memo = HashMap::new();

    println!("{}", rec(n, &mut memo));
}

fn rec(x: usize, memo: &mut HashMap<usize, mint>) -> mint {
    if x == 1 {
        return mint::from(1);
    }

    if let Some(&value) = memo.get(&x) {
        return value;
    }

    let mut p = mint::from(0);

    for i in 2..=6 {
        if x % i == 0 {
            p += mint::from(1) / mint::from(5) * rec(x / i, memo);
        }
    }

    memo.insert(x, p.clone());

    p
}
