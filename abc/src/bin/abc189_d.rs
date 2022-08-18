use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut dp = vec![vec![0isize; 2]; n + 1];

    // y0 が True となるようなものの、xの組の個数
    // x0 = True
    dp[0][1] = 1;

    // y0 が False となるようなものの、xの組の個数
    // x0 = False
    dp[0][0] = 1;

    // y1 が True となるようなものの、xの組の個数
    // s1 = AND
    // なし
    // dp[1][1] = 0;

    // y1 が False となるようなものの、xの組の個数
    // s1 = AND
    // x = [xxxx , True]
    // y = [False, False]
    // x = [False, False]
    // y = [False, False]
    // 2通り
    // dp[1][0] = 2;

    // y2 が True となるようなものの、xの組の個数
    // s2 = OR
    // x = [ xxxx, xxxx,  True]
    // y = [ xxxx, False, True]
    // x = [ xxxx, xxxx,  False]
    // y = [ xxxx, True,  True]

    // y2 が False となるようなものの、xの組の個数
    // s2 = OR
    // x = [ xxxx, xxxx,  False]
    // y = [ xxxx, False, False]

    for i in 1..=n {
        if s[i - 1] == "AND" {
            // yiがTrueになるような組み合わせ
            dp[i][1] += dp[i - 1][1]; // xi = True;
            dp[i][0] += dp[i - 1][1]; // xi = False;
            dp[i][0] += dp[i - 1][0] * 2; // xi = False;
        } else {
            // OR
            dp[i][1] += dp[i - 1][0]; // xi = True;
            dp[i][1] += dp[i - 1][1] * 2; // xi = True;
            dp[i][0] += dp[i - 1][0]; // xi = False;
        }
    }

    // println!("{:?}", dp);

    println!("{}", dp[n][1]);
}
