use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut result;

    let mut score = 0;

    // 一番左をリーダーにした場合
    for i in 1..n {
        if s[i] == 'E' {
            score += 1;
        }
    }

    result = score;

    for i in 1..n {
        if s[i] == 'E' {
            score -= 1;
        }

        if s[i - 1] == 'W' {
            score += 1;
        }

        result = result.min(score);
    }

    println!("{}", result);
}
