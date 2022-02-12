use std::io;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let q: usize = buf.parse().unwrap();

    for _ in 0..q {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let x: Vec<char> = buf.chars().collect();

        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let y: Vec<char> = buf.chars().collect();

        /* ---------------------------------- */

        let x_size = x.len();
        let y_size = y.len();

        let mut dp = vec![vec![-1; y_size + 1]; x_size + 1];

        for i in 0..=y_size {
            dp[0][i] = 0;
        }

        for i in 0..=x_size {
            dp[i][0] = 0;
        }

        for i in 0..x_size {
            for j in 0..y_size {
                if x[i] == y[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }

        println!("{}", dp[x_size][y_size]);

        /* ---------------------------------- */
    }
}
