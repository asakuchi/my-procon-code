use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut list = Vec::new();

    for i in 0..n {
        list.push((s[i], a[i]));
    }

    let mut result = 0_usize;

    for m_i in 0..=2 {
        for e_i in 0..=2 {
            for x_i in 0..=2 {
                let mex_str = vec![('M', m_i), ('E', e_i), ('X', x_i)];

                let mut dp = vec![vec![0; mex_str.len() + 1]; n + 1];

                for i in 0..=n {
                    // 1文字も使わない
                    dp[i][0] = 1;
                }

                for i in 1..=n {
                    for j in 1..=mex_str.len() {
                        dp[i][j] += dp[i - 1][j];

                        if list[i - 1] == mex_str[j - 1] {
                            dp[i][j] += dp[i - 1][j - 1];
                        }
                    }
                }

                result += mex(m_i, e_i, x_i) * dp[n][mex_str.len()];
            }
        }
    }

    println!("{}", result);
}

fn mex(a: usize, b: usize, c: usize) -> usize {
    for i in 0..=3 {
        if i != a && i != b && i != c {
            return i;
        }
    }

    100
}
