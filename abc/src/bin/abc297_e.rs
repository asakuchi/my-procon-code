use std::collections::BTreeSet;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut prices = BTreeSet::new();

    let mut priority_queue = BinaryHeap::new();

    for i in 0..n {
        priority_queue.push(Reverse(a[i]));
        prices.insert(a[i]);
    }

    while let Some(Reverse(price)) = priority_queue.pop() {
        for &tako_price in &a {
            let next_price = price + tako_price;

            if prices.contains(&next_price) {
                continue;
            }

            prices.insert(next_price);
            priority_queue.push(Reverse(next_price));
        }

        if prices.len() > k * 10 {
            break;
        }
    }

    if let Some(&k_th_price) = prices.iter().nth(k - 1) {
        println!("{}", k_th_price);
    } else {
        panic!();
    }
}
