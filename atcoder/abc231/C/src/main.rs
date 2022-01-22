// -*- coding:utf-8-unix -*-

use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();

    for question in x {
        let index = a.lower_bound(&question);

        let answer = n - index;

        println!("{}", answer);
    }
}
