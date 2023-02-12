use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; k],
    }

    let mut ng: HashSet<usize> = HashSet::new();

    for x in d {
        ng.insert(x);
    }

    'price_loop: for price in n..1_000_000 {
        let mut x = price;

        while x != 0 {
            let amari = x % 10;

            if ng.contains(&amari) {
                continue 'price_loop;
            }

            x = x / 10;
        }

        println!("{}", price);
        return;
    }
}
