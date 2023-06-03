use std::collections::HashMap;

use proconio::input;

use superslice::*;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        p_q: [(usize, usize); n],
        a_num: usize,
        a: [usize; a_num],
        b_num: usize,
        b: [usize; b_num],
    }

    let mut list_x = Vec::new();

    list_x.push(0);

    for &x in &a {
        list_x.push(x);
    }

    list_x.push(w);

    let mut list_y = Vec::new();

    list_y.push(0);

    for &y in &b {
        list_y.push(y);
    }

    list_y.push(h);

    let mut counter = HashMap::new();

    for i in 0..n {
        let (p, q) = p_q[i];

        let x = a.lower_bound(&p);
        let y = b.lower_bound(&q);

        *counter.entry((x, y)).or_insert(0_usize) += 1;
    }

    let max_m = counter.values().max().unwrap().clone();

    let min_m = if counter.len() == (a_num + 1) * (b_num + 1) {
        counter.values().min().unwrap().clone()
    } else {
        0
    };

    println!("{} {}", min_m, max_m);
}
