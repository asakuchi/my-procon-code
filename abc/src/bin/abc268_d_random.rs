use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let start = Instant::now();

    let mut rng = rand::thread_rng();

    let mut set = HashSet::new();

    for value in t {
        set.insert(value);
    }

    if n == 1 {
        if set.contains(&s[0]) {
            println!("-1");
            return;
        } else {
            if s[0].len() >= 3 && s[0].len() <= 16 {
                println!("{}", s[0]);
            } else {
                println!("-1");
            }

            return;
        }
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

    loop {
        // 時間制限
        let end = start.elapsed();
        if end.as_millis() >= 1800 {
            break;
        }

        let mut index_list: Vec<_> = (0..n).collect();

        // Vecを適当な順番に
        index_list.shuffle(&mut rng);

        // アンダースコアをバラつかせる
        let mut under_scores = vec![1; n - 1];

        // 選ぶアンダースコアの総数
        // Xの上限は16
        // そこからSの文字数を引き、
        // さらに最低一個は必要なので (n-1) を引く
        let underscore_total_count = 16 - s_total_len - (n - 1);

        // 何個アンダースコアを追加するかランダムに決める
        let target_underscore_count = rng.gen_range(0, underscore_total_count + 1);

        for _ in 0..target_underscore_count {
            let target = rng.gen_range(0, n - 1);

            under_scores[target] += 1;
        }

        let mut candi = Vec::new();

        for i in 0..n {
            if i != 0 {
                candi.push("_".repeat(under_scores[i - 1]));
            }

            candi.push(s[index_list[i]].clone());
        }

        let text = candi.iter().join("");

        if text.len() < 3 || text.len() > 16 {
            continue;
        }

        if set.contains(&text) {
            continue;
        }

        println!("{}", text);
        return;
    }

    println!("-1");
}
