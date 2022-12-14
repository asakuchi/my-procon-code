use proconio::{input, marker::Usize1};

const MOD: usize = 10000;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_b: [(Usize1, usize); k],
    }

    let mut must = vec![None; n];

    for (a, b) in a_b {
        must[a] = Some(b);
    }

    let mut dp = vec![vec![vec![None; 4]; 4]; n + 1];

    let result = rec(n, &must, 0, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    must: &Vec<Option<usize>>,
    index: usize,
    prev_pasta_1: usize,
    prev_pasta_2: usize,
    dp: &mut Vec<Vec<Vec<Option<usize>>>>,
) -> usize {
    if index == n {
        return 1;
    }

    if let Some(value) = dp[index][prev_pasta_1][prev_pasta_2] {
        return value;
    }

    let mut result = 0;

    if let Some(pasta) = must[index] {
        if pasta == prev_pasta_1 && pasta == prev_pasta_2 {
            // 3日連続
        } else {
            result += rec(n, must, index + 1, pasta, prev_pasta_1, dp);
        }
    } else {
        for pasta in 1..=3 {
            if pasta == prev_pasta_1 && pasta == prev_pasta_2 {
                // 3日連続
                continue;
            }

            result += rec(n, must, index + 1, pasta, prev_pasta_1, dp);
        }
    }

    result %= MOD;

    dp[index][prev_pasta_1][prev_pasta_2] = Some(result);

    result
}
