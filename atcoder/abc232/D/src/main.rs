// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    MyStruct { h, w, c }.main();
}

struct MyStruct {
    h: usize,
    w: usize,
    c: Vec<Vec<char>>,
}

impl MyStruct {
    fn main(&mut self) {
        let mut memo = vec![vec![-1; self.w]; self.h];

        if self.c[0][0] == '#' {
            println!("0");
            return;
        }

        let score = self.dfs((0, 0), &mut memo) + 1; // スタート地点で +1

        println!("{}", score);
    }

    fn dfs(&mut self, current: (usize, usize), memo: &mut Vec<Vec<i64>>) -> usize {
        if memo[current.0][current.1] != -1 {
            return memo[current.0][current.1] as usize;
        }

        // 右に移動
        let right_score = if current.1 + 1 >= self.w || self.c[current.0][current.1 + 1] == '#' {
            0
        } else {
            self.dfs((current.0, current.1 + 1), memo) + 1
        };

        // 下に移動
        let left_score = if current.0 + 1 >= self.h || self.c[current.0 + 1][current.1] == '#' {
            0
        } else {
            self.dfs((current.0 + 1, current.1), memo) + 1
        };

        let score = std::cmp::max(right_score, left_score);

        memo[current.0][current.1] = score as i64;

        score
    }
}
