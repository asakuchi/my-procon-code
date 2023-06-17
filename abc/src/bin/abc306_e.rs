use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        x_y: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];

    let mut top_k = 0_usize;

    let mut upper_value = BTreeSet::new();
    let mut lower_value = BTreeSet::new();

    for i in 0..k {
        // 暫定的に先頭k項を上位にする
        upper_value.insert((0, i));
    }

    for i in k..n {
        lower_value.insert((0, i));
    }

    for q_i in 0..q {
        let (x, y) = x_y[q_i];

        let old_value = a[x];

        a[x] = y;

        // f(A) を出す

        // old_valueが上位にいるなら更新
        if let Some(&(_value, i)) = upper_value
            .range((old_value, 0)..=(old_value, std::usize::MAX))
            .next()
        {
            upper_value.remove(&(old_value, i));

            // 暫定的に入れる
            lower_value.insert((y, n + q_i));

            if let Some(&(next_value, next_i)) = lower_value.iter().last() {
                upper_value.insert((next_value, next_i));
                lower_value.remove(&(next_value, next_i));

                top_k -= old_value;
                top_k += next_value;
            }
        } else {
            // 下位にいるはずなので更新
            if let Some(&(target_value, target_i)) = lower_value
                .range((old_value, 0)..=(old_value, std::usize::MAX))
                .next()
            {
                lower_value.remove(&(target_value, target_i));
            }

            // 新しい値は上位にいるべきか
            if let Some(&(less_value, less_i)) = upper_value.range(..(y, std::usize::MAX)).next() {
                // 下位 -> 上位

                upper_value.insert((y, n + q_i));

                // 繰り下がり
                upper_value.remove(&(less_value, less_i));
                lower_value.insert((less_value, less_i));

                top_k -= less_value;
                top_k += y;
            } else {
                // 下位 -> 下位

                // 下位のまま
                lower_value.insert((y, n + q_i));

                // このパターンはtop_kの更新なし
            }
        }

        println!("{}", top_k);
    }
}
