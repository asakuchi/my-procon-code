use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a_b: [(usize, usize); n],
    }

    let mut dp = vec![false; 10_100];

    dp[0] = true;

    for i in 0..n {
        let (a, b) = a_b[i];

        for _ in 0..b {
            let mut dp_next = dp.clone();

            for price in 1..=x {
                if dp[price] {
                    continue;
                }

                if price >= a {
                    if dp[price - a] {
                        dp_next[price] = true;
                    }
                }
            }

            std::mem::swap(&mut dp, &mut dp_next);
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
