use proconio::input;
use proconio::marker::Usize1;

const INF: isize = 20_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        s_t_d_time: [(Usize1, Usize1, isize, isize); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(s, t, d, time) in s_t_d_time.iter() {
        list[s].push((t, d, time));
        list[t].push((s, d, time));
    }

    let mut dp = vec![vec![(-1, 0); 1 << n]; n];

    let result = rec(n, &list, &mut dp, 0, 0);

    if result.1 != 0 {
        println!("{} {}", result.0, result.1);
    } else {
        println!("IMPOSSIBLE");
    }
}

fn rec(
    n: usize,
    list: &Vec<Vec<(usize, isize, isize)>>,
    dp: &mut Vec<Vec<(isize, usize)>>,
    current: usize,
    visited: usize,
) -> (isize, usize) {
    if visited == (1 << n) - 1 {
        if current == 0 {
            return (0, 1);
        } else {
            return (INF, 0);
        }
    }

    if dp[current][visited].0 != -1 {
        return dp[current][visited];
    }

    let mut answer = (INF, 0);

    for &(next, cost, time) in list[current].iter() {
        if visited & 1 << next > 0 {
            continue;
        }

        let (score, count) = rec(n, list, dp, next, visited | 1 << next);

        if score + cost > time {
            continue;
        }

        if answer.0 == score + cost {
            answer.1 += count;
        } else if answer.0 > score + cost {
            answer = (score + cost, count);
        }
    }

    dp[current][visited] = answer;

    answer
}
