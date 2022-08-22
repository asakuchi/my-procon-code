use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_b: [(usize, usize); n],
    }

    a_b.sort_by_key(|&(a, b)| Reverse(a * 2 + b));

    let mut aoki = 0;

    for &(a, _b) in &a_b {
        aoki += a;
    }

    let mut taka = 0;

    for (i, &(a, b)) in a_b.iter().enumerate() {
        taka += a + b;
        aoki -= a;

        // println!("a {} b {} taka:{} aoki:{}", a, b, taka, aoki);

        if taka > aoki {
            println!("{}", i + 1);
            return;
        }
    }
}
