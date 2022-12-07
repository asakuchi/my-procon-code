use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        toys: [Usize1; n],
    }

    let mut counter = HashMap::new();

    for &toy in &toys {
        *counter.entry(toy).or_insert(0) += 1;
    }

    let mut s = vec![vec![0; n + 1]; m];

    for i in 0..n {
        for j in 0..m {
            s[j][i + 1] = s[j][i] + if toys[i] == j { 1 } else { 0 };
        }
    }

    // println!("{:?}", s);

    let mut dp = vec![None; 1 << m];

    let result = rec(m, &s, &counter, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    m: usize,
    s: &Vec<Vec<usize>>,
    counter: &HashMap<usize, usize>,
    used: usize,
    dp: &mut Vec<Option<usize>>,
) -> usize {
    if used == (1 << m) - 1 {
        return 0;
    }

    if let Some(value) = dp[used] {
        return value;
    }

    let mut current_index = 0;

    // 現在どこまでチェックしたか調べる
    for i in 0..m {
        if used & 1 << i > 0 {
            let size = counter.get(&i).unwrap();
            current_index += size;
        }
    }

    let mut result = 1_000_000_000;

    for next in 0..m {
        if used & 1 << next > 0 {
            continue;
        }

        let size = counter.get(&next).unwrap();

        let toy_count = s[next][current_index + size] - s[next][current_index];

        // 違う種類のぬいぐるみを交換する
        let change = size - toy_count;

        let score = rec(m, s, counter, used | 1 << next, dp) + change;

        result = result.min(score);
    }

    dp[used] = Some(result);

    result
}
