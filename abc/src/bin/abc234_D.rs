// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    // let mut list = Vec::new();
    let mut list = BinaryHeap::new();

    for &data in p.iter().take(k) {
        list.push(Reverse(data));
    }

    // println!("a_list:{:?}", list);
    println!("{}", list.peek().unwrap().0);

    for i in k..n {
        // println!("i:{} k:{} n:{}", i, k, n);

        let data = p[i];

        let item = list.peek().unwrap();

        // println!("data:{:?} item:{:?}", data, item);

        if data > item.0 {
            list.pop();
            list.push(Reverse(data));
        }

        println!("{}", list.peek().unwrap().0);

        // println!("list:{:?}", list);
    }
}
