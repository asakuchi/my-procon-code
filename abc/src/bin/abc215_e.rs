use proconio::{input, marker::Chars};

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![vec![None; 11]; 1025]; n + 1];

    let result = rec(n, &s, 0, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    s: &Vec<char>,
    index: usize,
    used: usize,
    prev: usize,
    dp: &mut Vec<Vec<Vec<Option<mint>>>>,
) -> mint {
    if index == n {
        if used != 0 {
            return 1.into();
        } else {
            return 0.into();
        }
    }

    if let Some(value) = dp[index][used][prev] {
        return value;
    }

    let mut result = mint::from(0);

    let contest = s[index] as usize - 'A' as usize;

    // 出る
    if contest == prev || used & 1 << contest == 0 {
        result += rec(n, s, index + 1, used | 1 << contest, contest, dp);
    }

    // 出ない
    result += rec(n, s, index + 1, used, prev, dp);

    dp[index][used][prev] = Some(result);

    result
}
