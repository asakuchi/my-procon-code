// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    // let mut list = Vec::new();
    let mut list = std::collections::BinaryHeap::new();

    for (i, data) in p.iter().enumerate() {
        if i < k - 1 {
            list.push(data);
            continue;
        }

        list.push(data);

        let mut temp_list = Vec::new();

        for _ in 0..k - 1 {
            let item = list.pop();
            temp_list.push(item.unwrap());
        }

        let k_th = list.pop();
        temp_list.push(k_th.unwrap());

        println!("{}", k_th.unwrap());

        while let Some(value) = temp_list.pop() {
            list.push(value);
        }

        // println!("{:?}", list);
    }
}
