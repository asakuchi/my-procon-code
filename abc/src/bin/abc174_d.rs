use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut total_red = 0;
    let mut total_white = 0;

    for &stone in &c {
        if stone == 'R' {
            total_red += 1;
        }
    }

    total_white = n - total_red;

    let mut left_red = 0;
    let mut left_white = 0;

    let mut result = 1_000_000_000_000_000;

    for i in 0..n + 1 {
        let mut right_red = total_red - left_red;
        let mut right_white = total_white - left_white;

        let mut score = 0;

        if right_red > left_white {
            score += left_white;
            score += right_red - left_white;
        } else {
            score += right_red;
            score += left_white - right_red;
        }

        result = result.min(score);

        if i == n {
            break;
        }

        if c[i] == 'R' {
            left_red += 1;
        } else {
            left_white += 1;
        }
    }

    println!("{}", result);
}
