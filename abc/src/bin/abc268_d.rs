use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut set = HashSet::new();

    for value in t {
        set.insert(value);
    }

    // sに含まれる文字列の長さの合計
    let mut s_total_len = 0;

    for i in 0..n {
        s_total_len += s[i].len();
    }

    if s_total_len > 16 {
        println!("-1");
        return;
    }

    // 選ぶアンダースコアの総数
    // Xの上限は16
    // そこからSの文字数を引き、
    // さらに最低一個は必要なので (n-1) を引く
    let underscore_total_count = 16 - s_total_len - (n - 1);

    for permutation in (0..n).permutations(n) {
        let perm_s: Vec<_> = permutation.iter().map(|&i| s[i].clone()).collect();

        let candi = Vec::new();

        if rec(n, &perm_s, &set, 0, &candi, underscore_total_count) {
            return;
        }
    }

    println!("-1");
}

fn rec(
    n: usize,
    s: &Vec<String>,
    set: &HashSet<String>,
    current: usize,
    candi: &Vec<String>,
    remain: usize,
) -> bool {
    if current == n {
        return check(candi, set);
    }

    // remain は余分に使えるアンダースコアの数

    for i in 0..=remain {
        let mut new_candi = candi.clone();

        if current != 0 {
            new_candi.push("_".repeat(1 + i));
        }
        new_candi.push(s[current].clone());

        if rec(n, s, set, current + 1, &new_candi, remain - i) {
            return true;
        }
    }

    false
}

fn check(list: &Vec<String>, set: &HashSet<String>) -> bool {
    let text = list.iter().join("");

    if text.len() < 3 || text.len() > 16 {
        return false;
    }

    if set.contains(&text) {
        return false;
    }

    println!("{}", text);

    true
}
