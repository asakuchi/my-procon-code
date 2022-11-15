use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();

    let mut result = vec![0; n];

    // RL を探す
    // ここでぶつかる

    for i in 0..n - 1 {
        if i < n - 1 && s[i] == 'R' && s[i + 1] == 'L' {
            // ぶつかっている
            let mut switch = true;

            // 左方向
            for j in (0..=i).rev() {
                if s[j] != 'R' {
                    break;
                }

                if switch {
                    result[i] += 1;
                } else {
                    result[i + 1] += 1;
                }

                switch = !switch;
            }

            let mut switch = true;

            // 右方向
            for j in i + 1..n {
                if s[j] != 'L' {
                    break;
                }

                if switch {
                    result[i + 1] += 1;
                } else {
                    result[i] += 1;
                }

                switch = !switch;
            }
        }
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
