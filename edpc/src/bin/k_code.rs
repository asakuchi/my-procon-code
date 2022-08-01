use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; k + 1]; 3];

    dp[1][0] = 2;
    dp[2][0] = 1;

    for j in 0..=k {
        dp[1][j] = 2;
        dp[2][j] = 1;
    }

    for j in 0..=k {
        for player in 1..=2 {
            for i in 0..n {
                let rival = if player == 1 { 2 } else { 1 };
                let pre_result = dp[rival][j];

                if j + a[i] <= k && dp[player][j + a[i]] != player {
                    dp[player][j + a[i]] = pre_result;

                    // println!(
                    //     "j:{j} player:{player} a[i]:{} to:{} from:{}",
                    //     a[i],
                    //     dp[player][j + a[i]],
                    //     pre_result
                    // );
                }
            }
        }
    }

    // println!("{:?}", dp);

    if dp[1][k] == 1 {
        println!("First");
    } else if dp[1][k] == 2 {
        println!("Second");
    } else {
        panic!("error");
    }
}
