// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
    }

    let answer = dfs(n, k, &a, 0, 0, false, false);

    println!("{}", answer);
}

fn dfs(
    n: usize,
    k: isize,
    a: &Vec<isize>,
    index: usize,
    sum: isize,
    started: bool,
    ended: bool,
) -> usize {
    if index == n || ended {
        if sum == k {
            return 1;
        } else {
            return 0;
        }
    }

    let mut answer = 0;

    if started {
        if index != n - 1 {
            answer += dfs(n, k, a, index + 1, sum + a[index], true, false);
        }
        answer += dfs(n, k, a, index + 1, sum + a[index], true, true);
    } else {
        answer += dfs(n, k, a, index + 1, sum, false, false);

        if index != n - 1 {
            answer += dfs(n, k, a, index + 1, sum + a[index], true, false);
        }

        answer += dfs(n, k, a, index + 1, sum + a[index], true, true);
    }

    answer
}
