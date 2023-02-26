use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut list = vec![Vec::new(); n];
    let mut reverse_list = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '1' {
                list[i].push(i + j + 1);
                reverse_list[i + j + 1].push(i);
            }
        }
    }

    let mut top = vec![INF; n];
    let mut tail = vec![INF; n];

    top[0] = 0;
    tail[n - 1] = 0;

    for i in 0..n - 1 {
        for &j in list[i].iter() {
            top[j] = top[j].min(top[i] + 1);
        }
    }

    for i in (1..n).rev() {
        for &j in reverse_list[i].iter() {
            tail[j] = tail[j].min(tail[i] + 1);
        }
    }

    // println!("top {:?}", top);
    // println!("top {:?}", tail);

    let mut result = Vec::new();

    for k in 1..n - 1 {
        // k - m + 1 から
        // k を飛ばして
        // k + m - 1 まで

        let mut min_score = INF;

        for diff in 1..=m {
            let from = if k >= diff { k - diff } else { 0 };

            for &to in list[from].iter() {
                if to <= k {
                    continue;
                }

                let score = top[from] + tail[to] + 1;

                min_score = min_score.min(score);
            }
        }

        result.push(min_score);
    }

    let text = result
        .iter()
        .map(|&x| {
            if x != INF {
                x.to_string()
            } else {
                "-1".to_string()
            }
        })
        .format(" ");

    println!("{}", text);
}
