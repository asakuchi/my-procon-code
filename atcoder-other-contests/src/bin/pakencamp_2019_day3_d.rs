use proconio::{input, marker::Chars};

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        s: [Chars; 5],
    }

    let mut dp = vec![vec![None; 3]; n + 1];

    let result = rec(n, &s, 0, None, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    s: &Vec<Vec<char>>,
    index: usize,
    prev_color: Option<usize>,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == n {
        return 0;
    }

    if let Some(color) = prev_color {
        if let Some(value) = dp[index][color] {
            return value;
        }
    }

    let mut cost = vec![5; 3];

    for i in 0..5 {
        match s[i][index] {
            'R' => {
                cost[0] -= 1;
            }
            'B' => {
                cost[1] -= 1;
            }
            'W' => {
                cost[2] -= 1;
            }
            _ => {
                // do nothing.
            }
        }
    }

    let mut result = INF;

    for color in 0..=2 {
        if let Some(prev) = prev_color {
            if color == prev {
                continue;
            }
        }

        let score = rec(n, s, index + 1, Some(color), dp) + cost[color];

        result = result.min(score);
    }

    if let Some(color) = prev_color {
        dp[index][color] = Some(result);
    }

    result
}
