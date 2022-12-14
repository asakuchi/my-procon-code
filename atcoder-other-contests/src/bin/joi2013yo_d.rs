use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        t: [usize; d],
        a_b_c: [(usize, usize, usize); n],
    }

    // dp[日にち][派手さ]
    let mut dp = vec![vec![None; 101]; d + 1];

    let result = rec(d, &t, &a_b_c, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    d: usize,
    t: &Vec<usize>,
    a_b_c: &Vec<(usize, usize, usize)>,
    index: usize,
    prev_hade: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if index == d {
        return 0;
    }

    if let Some(value) = dp[index][prev_hade] {
        return value;
    }

    let mut result = 0;

    for &(a, b, c) in a_b_c {
        if t[index] < a || t[index] > b {
            continue;
        }

        let hade = if index == 0 {
            0
        } else {
            (prev_hade as isize - c as isize).abs() as usize
        };

        let score = rec(d, t, a_b_c, index + 1, c, dp) + hade;

        result = result.max(score);
    }

    dp[index][prev_hade] = Some(result);

    result
}
