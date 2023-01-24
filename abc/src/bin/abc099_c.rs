use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // for i in 1..=10 {
    //     println!("{}", i);
    //     println!("6 {}", 6usize.pow(i));
    //     println!("9 {}", 9usize.pow(i));
    // }

    // 7乗で100000を超える

    let mut dp = vec![1_000_000_000; n + 1];

    dp[0] = 0;

    for money in 1..=n {
        // 1円づつ
        dp[money] = dp[money].min(dp[money - 1] + 1);

        // 6^i円づつ
        for i in 1..10 {
            if money >= 6usize.pow(i) {
                dp[money] = dp[money].min(dp[money - 6usize.pow(i)] + 1);
            }
        }

        // 9^i円づつ
        for i in 1..10 {
            if money >= 9usize.pow(i) {
                dp[money] = dp[money].min(dp[money - 9usize.pow(i)] + 1);
            }
        }
    }

    println!("{}", dp[n]);
}
