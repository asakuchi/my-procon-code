use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![None; 21]; n];

    // let result = rec(n, &a, 0, 0, &mut dp);
    let result = rec(n, &a, 1, a[0], &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    index: usize,
    total: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == n - 1 {
        if total == a[n - 1] {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(value) = dp[index][total] {
        return value;
    }

    // +
    let score_1 = if total + a[index] <= 20 {
        rec(n, a, index + 1, total + a[index], dp)
    } else {
        0
    };

    // -
    let score_2 = if total >= a[index] {
        rec(n, a, index + 1, total - a[index], dp)
    } else {
        0
    };

    let result = score_1 + score_2;

    dp[index][total] = Some(result);

    result
}
