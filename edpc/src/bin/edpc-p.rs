use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        x_y: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(x, y) in &x_y {
        list[x].push(y);
        list[y].push(x);
    }

    let mut dp = vec![vec![-1; 2]; n];

    let black = rec(n, &list, 0, 1, 0, &mut dp);

    let white = rec(n, &list, 0, 0, 0, &mut dp);

    println!("{}", (white + black) % MOD);
}

fn rec(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    color: usize,
    prev: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if dp[current][color] != -1 {
        return dp[current][color] as usize;
    }

    let mut result = 1;

    for &next in list[current].iter() {
        if next == prev {
            continue;
        }

        let mut score = 0;

        if color == 0 {
            score += rec(n, list, next, 1, current, dp);
        }

        score += rec(n, list, next, 0, current, dp);

        result *= score;
        result %= MOD;
    }

    dp[current][color] = result as isize;

    result
}
