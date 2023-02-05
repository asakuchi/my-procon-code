use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        c: [usize; n],
        x: [Usize1; m],
    }

    let mut must_buy = vec![false; n];

    for value in x {
        must_buy[value] = true;
    }

    // println!("{:?}", must_buy);

    let mut min_c = vec![vec![1_000_000_000_000_000_000; n]; n];

    for l in 0..n {
        let mut min_value = c[l];

        for r in l..n {
            min_value = min_value.min(c[r]);
            min_c[l][r] = min_value;
        }
    }

    // println!("{:?}", min_c);

    // dp[i][j]
    // 商品 i-1 までは買うかどうかをすでに決定しており、
    // そのうち買うことにした商品の個数が j であるときの、
    // それまでに確定した合計費用としてあり得る最小値

    let mut dp = vec![vec![None; n]; n + 1];

    let result = rec(n, m, &a, &min_c, &must_buy, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    a: &Vec<usize>,
    min_c: &Vec<Vec<usize>>,
    must_buy: &Vec<bool>,
    i: usize,
    j: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if i == n {
        return 0;
    }

    if let Some(value) = dp[i][j] {
        return value;
    }

    // println!("i {} j {}", i, j);

    let mut result = 1_000_000_000_000_000_000;

    // 買う
    let score_1 = rec(n, m, a, min_c, must_buy, i + 1, j + 1, dp) + a[i] + min_c[i - j][i];

    result = result.min(score_1);

    // 買わない
    let score_2 = if !must_buy[i] {
        rec(n, m, a, min_c, must_buy, i + 1, j, dp)
    } else {
        1_000_000_000_000_000_000
    };

    result = result.min(score_2);

    // println!("result i {} j {} cost {}", i, j, result);

    dp[i][j] = Some(result);

    result
}
