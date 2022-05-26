use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const INF: usize = 1_000_000_000_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, [Usize1]); m],
    }

    let mut memo = vec![vec![-1; 1 << 13]; 1_100];

    let result = rec(n, m, 0, &abc, 0, &mut memo);

    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}

fn rec(
    n: usize,
    m: usize,
    current: usize,
    abc: &Vec<(usize, Vec<usize>)>,
    mask: usize,
    memo: &mut Vec<Vec<isize>>,
) -> usize {
    if m == current {
        // (1 << n) - 1 ⇒ 全部1
        if mask & (1 << n) - 1 == (1 << n) - 1 {
            return 0;
        } else {
            // 条件を満たしていない
            return INF;
        }
    }

    if memo[current][mask] != -1 {
        return memo[current][mask] as usize;
    }

    let mut next_mask = mask;

    for &treasure in abc[current].1.iter() {
        next_mask |= 1 << treasure;
    }

    let buy = rec(n, m, current + 1, abc, next_mask, memo) + abc[current].0;
    let not_buy = rec(n, m, current + 1, abc, mask, memo);

    let result = buy.min(not_buy);

    memo[current][mask] = result as isize;

    result
}
