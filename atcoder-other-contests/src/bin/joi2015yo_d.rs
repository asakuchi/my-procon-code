use proconio::input;

const INF: usize = 1_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: [usize; n],
        c: [usize; m],
    }

    // dp[day][town]
    let mut dp = vec![vec![None; n]; m];

    let result = rec(n, m, &d, &c, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    d: &Vec<usize>,
    c: &Vec<usize>,
    day: usize,
    town: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if town == n {
        return 0;
    }

    if day == m {
        return INF;
    }

    if let Some(value) = dp[day][town] {
        return value;
    }

    // go to next town
    let score_1 = rec(n, m, d, c, day + 1, town + 1, dp) + d[town] * c[day];

    // stay at current town
    let score_2 = rec(n, m, d, c, day + 1, town, dp);

    let result = score_1.min(score_2);

    dp[day][town] = Some(result);

    result
}
