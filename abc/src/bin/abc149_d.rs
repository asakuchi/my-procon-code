// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars,
    }

    MyStruct { n, k, r, s, p, t }.main();
}

struct MyStruct {
    n: usize,
    k: usize,
    r: usize,
    s: usize,
    p: usize,
    t: Vec<char>,
}

impl MyStruct {
    fn main(&mut self) {
        let mut score = 0;

        let mut hands = Vec::with_capacity(self.k);

        for _ in 0..self.k {
            hands.push(Vec::with_capacity(self.n / self.k + 1));
        }

        for i in 0..self.n {
            hands[i % self.k].push(self.t[i]);
        }

        for i in 0..self.k {
            let mut memo = vec![vec![-1; 4]; self.n / self.k + 1];
            score += self.dfs(0, 0, &hands[i], &mut memo);
        }

        println!("{}", score);
    }

    fn dfs(
        &mut self,
        index: usize,
        prev: usize,
        hands: &Vec<char>,
        memo: &mut Vec<Vec<isize>>,
    ) -> usize {
        // 終了条件
        if index >= hands.len() {
            return 0;
        }

        if memo[index][prev] != -1 {
            return memo[index][prev] as usize;
        }

        // 次に進む
        // 1:r
        // 2:s
        // 3:p
        let r_score = if prev != 1 {
            let win_score = if hands[index] == 's' { self.r } else { 0 };

            self.dfs(index + 1, 1, hands, memo) + win_score
        } else {
            0
        };

        let s_score = if prev != 2 {
            let win_score = if hands[index] == 'p' { self.s } else { 0 };

            self.dfs(index + 1, 2, hands, memo) + win_score
        } else {
            0
        };

        let p_score = if prev != 3 {
            let win_score = if hands[index] == 'r' { self.p } else { 0 };

            self.dfs(index + 1, 3, hands, memo) + win_score
        } else {
            0
        };

        use std::cmp::max;

        let score = max(p_score, max(r_score, s_score));

        memo[index][prev] = score as isize;

        score
    }
}
