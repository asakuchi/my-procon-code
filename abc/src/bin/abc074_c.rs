use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }

    let mut dp = vec![vec![None; 2_000]; 50];

    let result = rec(a, b, c, d, e, f, 0, 0, &mut dp);

    println!("{} {}", result.0 * 100 + result.1, result.1);
}

fn rec(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    water: usize,
    sugar: usize,
    dp: &mut Vec<Vec<Option<(usize, usize)>>>,
) -> (usize, usize) {
    if sugar > water * e {
        // 溶けきっていない
        return (0, 0);
    }

    let total = water * 100 + sugar;

    if total > f {
        // ビーカーが溢れる
        return (0, 0);
    }

    if let Some(value) = dp[water][sugar] {
        return value;
    }

    let mut result = (water, sugar);

    // println!(
    //     "water {} sugar {} total {} 濃度 {:?}",
    //     water, sugar, total, result,
    // );

    // ope 1
    let score = if total + 100 * a <= f {
        rec(a, b, c, d, e, f, water + a, sugar, dp)
    } else {
        (0, 0)
    };

    // 濃度が濃いなら更新
    result = update(result, score);

    // ope 2
    let score = if total + 100 * b <= f {
        rec(a, b, c, d, e, f, water + b, sugar, dp)
    } else {
        (0, 0)
    };

    result = update(result, score);

    // ope 3
    let score = if total + c <= f {
        rec(a, b, c, d, e, f, water, sugar + c, dp)
    } else {
        (0, 0)
    };

    result = update(result, score);

    // ope 4
    let score = if total + d <= f {
        rec(a, b, c, d, e, f, water, sugar + d, dp)
    } else {
        (0, 0)
    };

    result = update(result, score);

    dp[water][sugar] = Some(result);

    result
}

fn update(lhs: (usize, usize), rhs: (usize, usize)) -> (usize, usize) {
    // 濃度が濃いなら更新
    // (100 * rhs.1 / (rhs.0 + rhs.1)) > (100 * lhs.1 / (lhs.0 + lhs.1))

    if lhs == (0, 0) {
        rhs
    } else if rhs != (0, 0) && rhs.1 * (lhs.0 + lhs.1) > lhs.1 * (rhs.0 + rhs.1) {
        rhs
    } else {
        lhs
    }
}
