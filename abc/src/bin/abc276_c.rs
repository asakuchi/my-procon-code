use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut candidate = Vec::new();

    for i in 0..n {
        let mut copy = a.clone();

        let target = a[i];

        if target == 1 {
            continue;
        }

        let mut swap_target_index = None;
        let mut swap_target = 0;

        for j in i + 1..n {
            // 探す対象を1小さいではなく、
            // if a[j] == target - 1 {

            // target より小さい中で一番大きい
            if a[j] < target && a[j] > swap_target {
                swap_target_index = Some(j);
                swap_target = a[j];
                // break;
            }
        }

        if let None = swap_target_index {
            continue;
        }

        let swap_target_index = swap_target_index.unwrap();

        copy.swap(i, swap_target_index);

        copy[i + 1..].sort_by_key(|&x| Reverse(x));

        candidate.push(copy);
    }

    candidate.sort();

    // println!("{:?}", candidate);

    let mut result_index = 0;

    for i in 0..candidate.len() {
        if candidate[i] < a {
            result_index = i;
        } else {
            break;
        }
    }

    // println!("{:?}", candidate[result_index]);

    let text = candidate[result_index].iter().format(" ");
    println!("{}", text);
}
