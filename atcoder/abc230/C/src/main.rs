// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use std::cmp::max;
use std::cmp::min;

const BLACK: &str = "#";
const WHITE: &str = ".";

#[fastout]
fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128,
        p: i128,
        q: i128,
        r: i128,
        s: i128,
    }

    let mut map = vec![vec![WHITE; (s - r + 1) as usize]; (q - p + 1) as usize];

    for k in max(p - a, r - b)..=min(q - a, s - b) {
        if (a + k - p) < 0
            || (a + k - p) >= (q - p + 1)
            || (b + k - r) < 0
            || (b + k - r) >= (s - r + 1)
        {
            continue;
        }
        map[(a + k - p) as usize][(b + k - r) as usize] = BLACK;
    }

    for k in max(p - a, b - n)..=min(q - a, b - r) {
        if (a + k - p) < 0
            || (a + k - p) >= (q - p + 1)
            || (b - k - r) < 0
            || (b - k - r) >= (s - r + 1)
        {
            continue;
        }
        map[(a + k - p) as usize][(b - k - r) as usize] = BLACK;
    }

    for i in 0..q - p + 1 {
        for j in 0..s - r + 1 {
            print!("{}", map[i as usize][j as usize]);
        }
        println!();
    }
}
