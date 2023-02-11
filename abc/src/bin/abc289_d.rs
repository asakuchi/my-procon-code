use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut mochi = vec![false; 100_100];

    for i in b {
        mochi[i] = true;
    }

    let mut dp = vec![None; 100_100];

    if rec(n, &a, &mochi, x, 0, &mut dp) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    mochi: &Vec<bool>,
    x: usize,
    current: usize,
    dp: &mut Vec<Option<bool>>,
) -> bool {
    if current == x {
        return true;
    }

    if mochi[current] {
        return false;
    }

    if let Some(value) = dp[current] {
        return value;
    }

    for &step in a.iter() {
        if current + step > x {
            continue;
        }

        if rec(n, a, mochi, x, current + step, dp) {
            return true;
        }
    }

    dp[current] = Some(false);

    false
}
