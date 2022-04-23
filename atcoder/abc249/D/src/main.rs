use proconio::fastout;
use proconio::input;
use superslice::*;

// use proconio::derive_readable;
// use proconio::marker::Chars;
// use proconio::marker::Usize1;
// use itertools::izip;
// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;

use num_integer::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    // a.sort_by_key(|&x| std::cmp::Reverse(x));

    eprintln!("{:?}", a);

    let mut result = 0;

    for i in 0..n {
        // let mut j = 0;

        // while j < n && a[j] <= a[i] {
        //     if a[i] % a[j] != 0 {
        //         j += 1;
        //         continue;
        //     }

        //     let target = a[i] / a[j];

        //     let k_first = a.lower_bound(&target);

        //     for k in k_first..n {
        //         if k < n && a[i] == a[j] * a[k] {
        //             // eprintln!("{} {} {}", i, j, k);
        //             result += 1;
        //         } else {
        //             break;
        //         }
        //     }

        //     j += 1;
        // }

        // let half = ((a[i] + 1) / 2) + 3;
        let half = ((a[i] + 1) as f64).sqrt();

        let j_max = a.lower_bound(&((half as usize) + 1));

        for j in 0..=j_max {
            if a[i] % a[j] != 0 {
                continue;
            }

            let target = a[i] / a[j];

            let k_first = a.lower_bound(&target);

            for k in k_first..n {
                if k < n && a[i] == a[j] * a[k] {
                    eprintln!("{} {} {}", i, j, k);
                    result += 1;
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", result);
}
