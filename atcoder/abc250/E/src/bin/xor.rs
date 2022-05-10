//!
//! XOR ver
//!
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }

    let mut rng = rand::thread_rng();

    let mut set = HashSet::new();
    let mut map = HashMap::new();

    for i in 0..n {
        if set.contains(&a[i]) {
            a[i] = 0;
        } else {
            set.insert(a[i]);
            let num = rng.gen_range(0, 2usize.pow(32));
            map.insert(a[i], num);
            a[i] = num;
        }
    }

    let mut set = HashSet::new();

    for i in 0..n {
        if set.contains(&b[i]) {
            b[i] = 0;
        } else {
            set.insert(b[i]);

            if map.contains_key(&b[i]) {
                b[i] = *map.get(&b[i]).unwrap();
            } else {
                b[i] = rng.gen_range(0, 2usize.pow(32));
            }
        }
    }

    let mut acc_a = vec![0; n + 1];

    for i in 0..n {
        acc_a[i + 1] += acc_a[i] + a[i];
    }

    let mut acc_b = vec![0; n + 1];

    for i in 0..n {
        acc_b[i + 1] += acc_b[i] + b[i];
    }

    for &(x, y) in xy.iter() {
        if acc_a[x + 1] == acc_b[y + 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
