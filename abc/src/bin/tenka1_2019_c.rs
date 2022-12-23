use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // 白黒の順にしたい

    let mut right_black = 0;

    for &c in &s {
        if c == '#' {
            right_black += 1;
        }
    }

    let mut right_white = n - right_black;

    let mut result = n;

    // 境界線が一番左
    // 全て白
    result = result.min(right_black);

    // 全て黒
    result = result.min(right_white);

    let mut left_black = 0;
    // let mut left_white = 0;

    for &c in &s {
        if c == '#' {
            left_black += 1;
            right_black -= 1;
        } else {
            // left_white += 1;
            right_white -= 1;
        }

        let score = left_black + right_white;

        result = result.min(score);
    }

    println!("{}", result);
}
