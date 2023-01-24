use proconio::input;

const INF: usize = 1_000_000_000_000;

fn main() {
    input! {
        d: usize,
        g: usize,
        p_c: [(usize, usize); d],
    }

    // dp[問題の種類][総合得点]
    let mut dp = vec![vec![None; 1_000_100]; d + 1];

    let result = rec(d, g, &p_c, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    d: usize,
    g: usize,
    p_c: &Vec<(usize, usize)>,
    index: usize,
    score: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == d {
        // 100分の1のスコアで管理
        if score >= g / 100 {
            return 0;
        } else {
            return INF;
        }
    }

    if let Some(value) = dp[index][score] {
        return value;
    }

    let mut result = INF;

    let (p, c) = p_c[index];

    for i in 0..=p {
        let question_num = rec(
            d,
            g,
            p_c,
            index + 1,
            // 100分の1のスコアで管理
            score + (index + 1) * i + if i == p { c / 100 } else { 0 },
            dp,
        ) + i;

        result = result.min(question_num);
    }

    dp[index][score] = Some(result);

    result
}
