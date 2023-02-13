use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut dp = vec![vec![None; 2]; n + 1];

    let mut result = 0;

    result += rec(n, &s, 1, true, &mut dp);
    result += rec(n, &s, 1, false, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    s: &Vec<String>,
    index: usize,
    y_i_1: bool,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(value) = dp[index][y_i_1 as usize] {
        return value;
    }

    let s_i = &s[index - 1];

    let mut result = 0;

    if index == n {
        if s_i == "AND" {
            if true && y_i_1 {
                result += 1;
            }

            if false && y_i_1 {
                result += 1;
            }
        } else {
            if true || y_i_1 {
                result += 1;
            }

            if false || y_i_1 {
                result += 1;
            }
        }

        return result;
    }

    if s_i == "AND" {
        // x_i True
        result += rec(n, s, index + 1, true && y_i_1, dp);

        // x_i False
        result += rec(n, s, index + 1, false && y_i_1, dp);
    } else {
        // x_i True
        result += rec(n, s, index + 1, true || y_i_1, dp);

        // x_i False
        result += rec(n, s, index + 1, false || y_i_1, dp);
    }

    dp[index][y_i_1 as usize] = Some(result);

    result
}
