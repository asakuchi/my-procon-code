use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
        mut a: [usize; k],
    }

    let mut dp = vec![vec![None; n + 1]; 2];

    // taka turn
    let result = rec(n, k, &a, n, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    k: usize,
    a: &Vec<usize>,
    rest_stone: usize,
    turn: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if rest_stone == 0 {
        return 0;
    }

    if let Some(value) = dp[turn][rest_stone] {
        return value;
    }

    let mut result: Option<usize> = None;

    for i in 0..k {
        let taking = a[i];

        if rest_stone < taking {
            continue;
        }

        let score_current = if turn % 2 == 0 { taking } else { 0 };

        let score = rec(n, k, a, rest_stone - taking, (turn + 1) % 2, dp) + score_current;

        if let Some(best_score) = result {
            if turn % 2 == 0 {
                // taka turn
                result = Some(best_score.max(score));
            } else {
                // aoki turn
                result = Some(best_score.min(score));
            }
        } else {
            result = Some(score);
        }
    }

    dp[turn][rest_stone] = result;

    result.unwrap()
}
