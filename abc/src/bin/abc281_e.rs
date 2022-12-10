use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = Vec::new();

    let mut total = 0;

    let mut smaller = BTreeSet::new();
    let mut bigger = BTreeSet::new();

    for i in 0..k {
        smaller.insert((a[i], i));

        total += a[i];
    }

    for i in k..m {
        if let Some(&(value, uniq)) = smaller.range((a[i] + 1, 0)..).next_back() {
            smaller.remove(&(value, uniq));
            smaller.insert((a[i], i));

            bigger.insert((value, uniq));

            total -= value;
            total += a[i];
        } else {
            bigger.insert((a[i], i));
        }
    }

    result.push(total);

    for i in 1..n - m + 1 {
        // a[i-1] を削除
        let delete_num = a[i - 1];

        // a[i+m]を追加
        let additive_num = a[i + m - 1];

        // smaller に入ってるなら
        // smallerから削除
        if let Some(&(value, uniq)) = smaller
            .range((delete_num, 0)..=(delete_num, 1_000_000))
            .next_back()
        {
            smaller.remove(&(value, uniq));
            total -= value;
        } else if let Some(&(value, uniq)) = bigger
            .range((delete_num, 0)..=(delete_num, 1_000_000))
            .next_back()
        {
            // biggerに入ってるなら
            // biggerから削除
            bigger.remove(&(value, uniq));
        }

        // smallerに追加するか
        if let Some((value, uniq)) = smaller.range((additive_num + 1, 0)..).next_back().copied() {
            smaller.remove(&(value, uniq));
            total -= value;

            smaller.insert((additive_num, i));
            total += additive_num;

            bigger.insert((value, uniq));
        } else {
            // smallerに追加しないならとりあえずbiggerに入れておく
            bigger.insert((additive_num, i));
        }

        // サイズを調整 smaller
        while smaller.len() < k {
            if let Some((bigger_smallest, smallest_uniq)) = bigger.iter().next().copied() {
                smaller.insert((bigger_smallest, smallest_uniq));
                total += bigger_smallest;

                bigger.remove(&(bigger_smallest, smallest_uniq));
            }
        }

        // サイズを調整 bigger
        while bigger.len() < m - k {
            if let Some(&(value, uniq)) = bigger.iter().next_back() {
                bigger.remove(&(value, uniq));
            }
        }

        result.push(total);
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
