use std::collections::BinaryHeap;
use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        t_x: [(usize, usize); n],
    }

    let mut normal = Vec::new();
    let mut need = Vec::new();
    let mut opener = Vec::new();

    for &(t, x) in &t_x {
        if t == 0 {
            normal.push(x);
        } else if t == 1 {
            need.push(x);
        } else {
            opener.push(x);
        }
    }

    normal.sort_by_key(|&x| Reverse(x));
    need.sort_by_key(|&x| Reverse(x));
    opener.sort();

    let mut result = 0_usize;

    let mut set = BTreeSet::new();

    let mut uniq_key = 0;

    // 缶切り不使用
    for i in 0..m.min(normal.len()) {
        result += normal[i];

        set.insert((normal[i], uniq_key));
        uniq_key += 1;
    }

    let mut priority_queue_2 = BinaryHeap::new();

    for x in need {
        priority_queue_2.push(x);
    }

    let mut score = result;

    while let Some(open) = opener.pop() {
        let mut rest_open = open;

        if m > 0 {
            m -= 1;
        }

        while set.len() > m {
            if let Some(x) = set.iter().next().copied() {
                set.remove(&x);

                score -= x.0;
            }
        }

        while rest_open > 0 {
            // オープナーいる
            if let Some(next_x) = priority_queue_2.pop() {
                set.insert((next_x, uniq_key));
                uniq_key += 1;

                score += next_x;

                while set.len() > m {
                    if let Some(x) = set.iter().next().copied() {
                        set.remove(&x);

                        score -= x.0;
                    }
                }
            } else {
                break;
            }

            rest_open -= 1;

            result = result.max(score);
        }
    }

    println!("{}", result);
}
