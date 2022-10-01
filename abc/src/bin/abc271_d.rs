use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; s + 1]; n + 1];
    let mut result = Vec::new();

    if rec(n, s, &a_b, 0, &mut dp, &mut result) {
        println!("Yes");

        result.reverse();

        let text = result
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");

        println!("{}", text);
    } else {
        println!("No");
    }
}

fn rec(
    n: usize,
    total: usize,
    a_b: &Vec<(usize, usize)>,
    index: usize,
    dp: &mut Vec<Vec<Option<bool>>>,
    result: &mut Vec<char>,
) -> bool {
    if index == n {
        return total == 0;
    }

    if let Some(value) = dp[index][total] {
        return value;
    }

    let (a, b) = a_b[index];

    // 表
    if total >= a {
        if rec(n, total - a, a_b, index + 1, dp, result) {
            result.push('H');
            return true;
        }
    }

    // 裏
    if total >= b {
        if rec(n, total - b, a_b, index + 1, dp, result) {
            result.push('T');
            return true;
        }
    }

    dp[index][total] = Some(false);

    false
}
