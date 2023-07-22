fn main() {
    let (h, w) = input_tuple();
    let c = input_vec_2d::<usize>(h);

    let mut dp = vec![vec![0; w + 1]; h + 1];

    let mut max_size = 0;

    for i in 1..=h {
        for j in 1..=w {
            if c[i - 1][j - 1] == 1 {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]) + 1;
            }

            max_size = max_size.max(dp[i][j]);
        }
    }

    println!("{}", max_size * max_size);
}

fn input_tuple<T>() -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_vec_2d<T>(n: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // 二次元ベクタ

    let stdin = std::io::stdin();

    let mut a = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let line = iter.map(|x| x.parse().unwrap()).collect();

        a.push(line);
    }

    a
}
