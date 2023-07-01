use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
        l: [usize; m],
        d: [usize; m],
    }

    let mut result = 0_usize;

    p.sort();

    let mut list = Vec::new();

    for i in 0..m {
        list.push((l[i], d[i]));
    }

    list.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    let mut queue = BinaryHeap::new();

    for i in 0..n {
        let mut price = p[i];

        while let Some((limit, down)) = list.pop() {
            if limit <= price {
                queue.push(down);
            } else {
                list.push((limit, down));

                break;
            }
        }

        if let Some(down) = queue.pop() {
            price -= down;
        }

        result += price;
    }

    println!("{}", result);
}
