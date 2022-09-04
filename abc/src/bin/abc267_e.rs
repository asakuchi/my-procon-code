use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        mut u_v: [(Usize1, Usize1); m],
    }

    // 隣接リスト
    let mut list = vec![HashSet::new(); n];

    for &(u, v) in &u_v {
        list[u].insert(v);
        list[v].insert(u);
    }

    let mut priority_queue = BinaryHeap::new();

    // 初期コストを計算
    for i in 0..n {
        let mut cost = 0;

        for &v in list[i].iter() {
            cost += a[v];
        }

        priority_queue.push(Reverse((cost, i)));
    }

    let mut deleted = HashSet::new();

    let mut result = 0;

    // コストの低い順に削除
    while let Some(Reverse((cost, v))) = priority_queue.pop() {
        if deleted.contains(&v) {
            continue;
        }

        deleted.insert(v);

        result = result.max(cost);

        for &next in list[v].clone().iter() {
            list[v].remove(&next);
            list[next].remove(&v);

            let mut cost = 0;

            // 頂点vを削除した後、vと直接結ばれている頂点のコストを計算
            for &next_next in list[next].iter() {
                cost += a[next_next];
            }

            priority_queue.push(Reverse((cost, next)));
        }
    }

    println!("{}", result);
}
