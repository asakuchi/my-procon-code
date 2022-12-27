use std::collections::HashMap;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for &value in &a {
        *map.entry(value).or_insert(0) += 1;
    }

    // let start_i = if n % 2 == 0 { 1 } else { 0 };

    if n % 2 != 0 {
        for i in (0..=n - 1).step_by(2) {
            if let Some(&count) = map.get(&i) {
                if i == 0 {
                    if count != 1 {
                        println!("0");
                        return;
                    }
                } else {
                    if count != 2 {
                        println!("0");
                        return;
                    }
                }
            } else {
                println!("0");
                return;
            }
        }
    } else {
        for i in (1..=n - 1).step_by(2) {
            if let Some(&count) = map.get(&i) {
                if count != 2 {
                    println!("0");
                    return;
                }
            } else {
                println!("0");
                return;
            }
        }
    }

    if n == 1 {
        println!("1");
        return;
    }

    let result = mod_pow(2, n / 2);

    println!("{}", result);
}

fn mod_pow(x: usize, a: usize) -> usize {
    if a == 1 {
        return x;
    }

    if a % 2 == 1 {
        return (x * mod_pow(x, a - 1)) % MOD;
    }

    let t = mod_pow(x, a / 2);

    return (t * t) % MOD;
}
