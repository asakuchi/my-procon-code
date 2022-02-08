use proconio::input;

///
/// ナップザック問題
/// 個数制限なし
///
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize,usize); n],
    }

    let mut dp = vec![vec![-1; w + 1]; n + 1];

    let result = rec(n, w, &wv, 0, w, &mut dp);

    println!("{:?}", dp);

    println!("{}", result);

    // for i in 0..n {
    //     for j in 0..=w {
    //         if j < wv[i].0 {
    //             dp[i + 1][j] = dp[i][j];
    //         } else {
    //             dp[i + 1][j] = std::cmp::max(dp[i][j], dp[i + 1][j - wv[i].0] + wv[i].1);
    //         }
    //     }
    // }

    // println!("{}", dp[n][w]);
}

fn rec(
    n: usize,
    w: usize,
    wv: &Vec<(usize, usize)>,
    i: usize,
    j: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if i == n {
        return 0;
    }

    if dp[i][j] != -1 {
        return dp[i][j] as usize;
    }

    let score = if j < wv[i].0 {
        let score = rec(n, w, wv, i + 1, j, dp);

        score
    } else {
        let score_1 = rec(n, w, wv, i + 1, j, dp);

        // 選択
        let score_2 = rec(n, w, wv, i, j - wv[i].0, dp) + wv[i].1;

        std::cmp::max(score_1, score_2)
    };

    dp[i][j] = score as isize;

    score
}
