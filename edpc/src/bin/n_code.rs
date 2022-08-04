use proconio::input;

const INF: isize = 1_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![(-1, 0); n + 1]; n + 1];

    let result = rec(n, &a, 0, n, &mut dp);

    println!("{}", result.0);
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    left: usize,
    right: usize,
    dp: &mut Vec<Vec<(isize, usize)>>,
) -> (isize, usize) {
    if left + 2 == right {
        return (
            a[left] as isize + a[right - 1] as isize,
            a[left] + a[right - 1],
        );
    }

    if left + 1 == right {
        return (0, a[left]);
    }

    if left + 1 >= right {
        return (INF, 0);
    }

    if dp[left][right].0 != -1 {
        return dp[left][right];
    }

    let mut result = (INF, 0);

    for i in left + 1..right {
        let score_1 = rec(n, a, left, i, dp);

        let score_2 = rec(n, a, i, right, dp);

        let score = score_1.0 + score_2.0 + score_1.1 as isize + score_2.1 as isize;
        let weight = score_1.1 + score_2.1;

        if result.0 > score {
            result = (score, weight);
        };
    }

    dp[left][right] = result;

    result
}
