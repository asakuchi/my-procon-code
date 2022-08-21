use std::collections::HashSet;

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: isize,
        b: isize,
        c: isize,
        d: isize,
        e: isize,
        f: isize,
        x_y: [(isize, isize); m],
    }

    let mut wall = HashSet::new();

    for &(x, y) in &x_y {
        wall.insert((x, y));
    }

    let mut dp = vec![vec![vec![Option::None; n]; n]; n];

    let result = rec(n, m, a, b, c, d, e, f, 0, 0, 0, &wall, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
    f: isize,
    step: usize,
    warp_1_count: usize,
    warp_2_count: usize,
    wall: &HashSet<(isize, isize)>,
    dp: &mut Vec<Vec<Vec<Option<usize>>>>,
) -> usize {
    let warp_3_count = step - warp_1_count - warp_2_count;

    let current = (
        a * warp_1_count as isize + c * warp_2_count as isize + e * warp_3_count as isize,
        b * warp_1_count as isize + d * warp_2_count as isize + f * warp_3_count as isize,
    );

    if wall.contains(&current) {
        return 0;
    }

    if step == n {
        return 1;
    }

    if let Some(value) = dp[step][warp_2_count][warp_3_count] {
        return value;
    }

    let count_1 = rec(
        n,
        m,
        a,
        b,
        c,
        d,
        e,
        f,
        step + 1,
        warp_1_count + 1,
        warp_2_count,
        wall,
        dp,
    );

    let count_2 = rec(
        n,
        m,
        a,
        b,
        c,
        d,
        e,
        f,
        step + 1,
        warp_1_count,
        warp_2_count + 1,
        wall,
        dp,
    );

    let count_3 = rec(
        n,
        m,
        a,
        b,
        c,
        d,
        e,
        f,
        step + 1,
        warp_1_count,
        warp_2_count,
        wall,
        dp,
    );

    let count = (count_1 + count_2 + count_3) % MOD;

    dp[step][warp_2_count][warp_3_count] = Some(count);

    count
}
