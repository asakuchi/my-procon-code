use std::io;

fn main() {
    let t = input();

    for _ in 0..t {
        let n = input();

        let mut dp = vec![0.; n + 10];

        for i in (0..n).rev() {
            for k in 1..=6 {
                dp[i] += (dp[i + k] + 1.) / 6.;
            }
        }
        println!("{}", dp[0]);
    }
}

fn input() -> usize {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}
