// -*- coding:utf-8-unix -*-

use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; 2 * n]; 2 * n];

    for i in 0..2 * n {
        for j in i + 1..2 * n {
            input! { value: usize }
            a[i][j] = value;
        }
    }

    // let perms = (0..2 * n).combinations(2);

    // // println!("{:?}", perms);

    // let list: Vec<Vec<usize>> = perms.collect();

    let mut score = 0;

    let mut memo = vec![vec![-1; 1000]; 1000000];

    for _ in 0..2 * n {
        let happiness = dfs(&a, n, 0, &mut memo);

        score = std::cmp::max(score, happiness);
    }

    // println!("{}", if yes { "Yes" } else { "No" });
    println!("{}", score);
}

fn dfs(a: &Vec<Vec<usize>>, n: usize, selected: usize, memo: &mut Vec<Vec<isize>>) -> usize {
    // if selected &&
    // println!("{}", selected);

    if memo[n][selected] != -1 {
        return memo[n][selected] as usize;
    }

    let mut rest = Vec::new();

    for i in 0..2 * n {
        if selected >> i & 1 == 0 {
            rest.push(i);
        }
    }

    if rest.is_empty() {
        return 0;
    }

    let com = rest.iter().combinations(2);

    let mut score = 0;

    for pair in com {
        let next = selected | (1 << pair[0]) as usize | (1 << pair[1]) as usize;

        // println!("next:{}", next);

        let next_score = dfs(a, n, next, memo) ^ a[*pair[0]][*pair[1]];

        score = std::cmp::max(score, next_score);
    }

    memo[n][selected] = score as isize;

    score
}
