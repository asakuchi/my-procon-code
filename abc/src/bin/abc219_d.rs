use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![vec![vec![None; 301]; 301]; 301];

    let result = rec(n, &a_b, 0, x, y, &mut dp);

    if result >= 1_000_000_000_000 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}

fn rec(
    n: usize,
    a_b: &Vec<(usize, usize)>,
    index: usize,
    tako: usize,
    tai: usize,
    dp: &mut Vec<Vec<Vec<Option<usize>>>>,
) -> usize {
    if index == n {
        return if tako == 0 && tai == 0 {
            0
        } else {
            1_000_000_000_000
        };
    }

    if let Some(value) = dp[index][tako][tai] {
        return value;
    }

    // 買う
    let result_1 = rec(
        n,
        a_b,
        index + 1,
        tako.saturating_sub(a_b[index].0),
        tai.saturating_sub(a_b[index].1),
        dp,
    ) + 1;

    // 買わない
    let result_2 = rec(n, a_b, index + 1, tako, tai, dp);

    let result = result_1.min(result_2);

    dp[index][tako][tai] = Some(result);

    result
}
