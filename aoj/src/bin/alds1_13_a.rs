//!
//! 8クイーン問題
//!

use std::io;

fn main() {
    let k = input_k();
    let n = 8;

    let mut row = vec![false; n];
    let mut col = vec![false; n];
    let mut naname_pos = vec![false; 2 * n - 1];
    let mut naname_neg = vec![false; 2 * n - 1];

    let mut list = Vec::new();

    for _ in 0..k {
        let (r, c) = input_rc();

        row[r] = true;
        col[c] = true;
        naname_pos[r + c] = true;
        naname_neg[n - 1 + r - c] = true;

        list.push((r, c));
    }

    dfs(
        n,
        k,
        &mut list,
        &mut row,
        &mut col,
        &mut naname_pos,
        &mut naname_neg,
    );

    let mut result = vec![vec!['.'; n]; n];

    for (r, c) in list {
        result[r][c] = 'Q';
    }

    for i in 0..n {
        println!(
            "{}",
            result[i].iter().map(|c| c.to_string()).collect::<String>()
        );
    }
}

fn input_k() -> usize {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let k: usize = buf.parse().unwrap();

    k
}

fn input_rc() -> (usize, usize) {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let r: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();

    (r, c)
}

fn dfs(
    n: usize,
    k: usize,
    list: &mut Vec<(usize, usize)>,
    row: &mut Vec<bool>,
    col: &mut Vec<bool>,
    naname_pos: &mut Vec<bool>,
    naname_neg: &mut Vec<bool>,
) -> bool {
    if list.len() == n {
        return true;
    }

    for i in 0..n {
        for j in 0..n {
            if row[i] || col[j] || naname_pos[i + j] || naname_neg[n - 1 + i - j] {
                continue;
            }

            row[i] = true;
            col[j] = true;
            naname_pos[i + j] = true;
            naname_neg[n - 1 + i - j] = true;

            list.push((i, j));

            let ok = dfs(n, k, list, row, col, naname_pos, naname_neg);

            if ok {
                return true;
            }

            row[i] = false;
            col[j] = false;
            naname_pos[i + j] = false;
            naname_neg[n - 1 + i - j] = false;

            list.pop();
        }
    }

    false
}
