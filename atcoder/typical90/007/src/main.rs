// -*- coding:utf-8-unix -*-

use proconio::input;
use superslice::*;

///
/// 007 - CP Classes（★3）
///
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q],
    }

    a.sort();

    for student in b {
        if student <= a[0] {
            println!("{}", a[0] - student);
            continue;
        }

        if student >= a[a.len() - 1] {
            println!("{}", student - a[a.len() - 1]);
            continue;
        }

        // let mut left = 0;
        // let mut right = a.len() - 1;

        // let mut middle = (left + right) / 2;

        // loop {
        //     // 生徒のレーティングがどこのクラス間か探す
        //     if a[middle] <= student && student < a[middle + 1] {
        //         break;
        //     }

        //     if student > a[middle] {
        //         left = middle;
        //     } else {
        //         right = middle;
        //     }

        //     middle = (left + right) / 2;
        // }

        let middle = a.lower_bound(&student);

        let score = std::cmp::min((a[middle - 1] - student).abs(), (a[middle] - student).abs());

        println!("{}", score);
    }
}
