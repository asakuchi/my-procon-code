use proconio::{input, marker::Chars};

use ac_library_rs::ModInt998244353 as mint;

fn main() {
    input! {
        s: Chars,
    }

    let mut dp = vec![vec![None; s.len()]; s.len() + 1];

    let result = rec(&s, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(s: &Vec<char>, index: usize, left_count: usize, dp: &mut Vec<Vec<Option<mint>>>) -> mint {
    if index == s.len() {
        if left_count == 0 {
            return mint::from(1);
        } else {
            return mint::from(0);
        }
    }

    if let Some(value) = dp[index][left_count] {
        return value;
    }

    let mut result = mint::from(0);

    if s[index] == '(' {
        result += rec(s, index + 1, left_count + 1, dp);
    } else if s[index] == ')' {
        if left_count > 0 {
            result += rec(s, index + 1, left_count - 1, dp);
        } else {
            // do nothing;
        }
    } else {
        if left_count > 0 {
            // ')'
            result += rec(s, index + 1, left_count - 1, dp);
        }
        // '('
        result += rec(s, index + 1, left_count + 1, dp);
    }

    dp[index][left_count] = Some(result);

    result
}
