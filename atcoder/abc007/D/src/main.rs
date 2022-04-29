use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let b_list: Vec<_> = b.to_string().chars().collect();
    let a_list: Vec<_> = (a - 1).to_string().chars().collect();

    println!("{}", forbidden_count(b_list) - forbidden_count(a_list));
}

fn forbidden_count(n: Vec<char>) -> usize {
    let mut dp = vec![vec![vec![0; 2]; 2]; n.len() + 1];

    dp[0][0][0] = 1;

    let size = n.len();

    for i in 0..size {
        let num: usize = n[i].to_string().parse().unwrap();

        for smaller in vec![false, true] {
            for has_4_or_9 in vec![false, true] {
                let mut x = 0;

                while x <= if smaller { 9 } else { num } {
                    dp[i + 1][(smaller || x < num) as usize]
                        [(has_4_or_9 || x == 4 || x == 9) as usize] +=
                        dp[i][smaller as usize][has_4_or_9 as usize];

                    x += 1;
                }
            }
        }
    }

    dp[size][0][1] + dp[size][1][1]
}
