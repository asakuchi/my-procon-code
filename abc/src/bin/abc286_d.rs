use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; 30_000]; n + 1];

    let result = rec(n, x, &a_b, 0, &mut dp);

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    x: usize,
    a_b: &Vec<(usize, usize)>,
    index: usize,
    dp: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if index == n {
        if x == 0 {
            return true;
        } else {
            return false;
        }
    }

    if let Some(value) = dp[index][x] {
        return value;
    }

    if x == 0 {
        return true;
    }

    let (a, b) = a_b[index];

    for i in 0..=b {
        if x < a * i {
            break;
        }

        let result = rec(n, x - a * i, a_b, index + 1, dp);

        if result {
            return true;
        }
    }

    dp[index][x] = Some(false);

    false
}
