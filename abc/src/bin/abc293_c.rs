use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let set = HashSet::new();

    let result = rec(h, w, &a, 0, 0, &set);

    println!("{}", result);
}

fn rec(h: usize, w: usize, a: &Vec<Vec<usize>>, i: usize, j: usize, set: &HashSet<usize>) -> usize {
    let mut new_set = set.clone();
    new_set.insert(a[i][j]);

    if i == h - 1 && j == w - 1 {
        if new_set.len() == h + w - 1 {
            return 1;
        } else {
            return 0;
        }
    }

    // 右
    let score_1 = if j < w - 1 {
        rec(h, w, a, i, j + 1, &new_set)
    } else {
        0
    };

    // 下
    let score_2 = if i < h - 1 {
        rec(h, w, a, i + 1, j, &new_set)
    } else {
        0
    };

    score_1 + score_2
}
