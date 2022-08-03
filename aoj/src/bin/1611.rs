use std::io;

fn main() {
    loop {
        // ------------------------------------
        let stdin = io::stdin();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let n: usize = buf.parse().unwrap();

        if n == 0 {
            break;
        }

        // ------------------------------------
        let stdin = io::stdin();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let w: Vec<usize> = iter.map(|x| x.parse().unwrap()).collect();

        // ------------------------------------

        let mut dp = vec![vec![-1; n + 1]; n + 1];

        let result = rec(n, &w, 0, n, &mut dp);

        println!("{}", result);
    }
}

fn rec(n: usize, w: &Vec<usize>, left: usize, right: usize, dp: &mut Vec<Vec<isize>>) -> usize {
    if left + 1 >= right {
        return 0;
    }

    if left + 2 == right {
        if (w[left] as isize - w[right - 1] as isize).abs() <= 1 {
            return 2;
        } else {
            return 0;
        }
    }

    if dp[left][right] != -1 {
        return dp[left][right] as usize;
    }

    let mut result = 0;

    // 全部取り除ける場合
    if (w[left] as isize - w[right - 1] as isize).abs() <= 1
        && rec(n, w, left + 1, right - 1, dp) == right - left - 2
    {
        result = result.max(right - left);
    }

    // その他
    for i in left + 1..right {
        let score = rec(n, w, left, i, dp) + rec(n, w, i, right, dp);

        result = result.max(score);
    }

    dp[left][right] = result as isize;

    result
}
