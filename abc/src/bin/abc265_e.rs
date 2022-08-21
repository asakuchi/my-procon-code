use std::collections::{HashMap, HashSet};

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

    let mut dp = vec![HashMap::new(); n];

    let result = rec(n, m, a, b, c, d, e, f, (0, 0), 0, &wall, &mut dp);

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
    current: (isize, isize),
    step: usize,
    wall: &HashSet<(isize, isize)>,
    dp: &mut Vec<HashMap<(isize, isize), usize>>,
) -> usize {
    if wall.contains(&current) {
        return 0;
    }

    if step == n {
        return 1;
    }

    if dp[step].contains_key(&current) {
        return *dp[step].get(&current).unwrap();
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
        (current.0 + a, current.1 + b),
        step + 1,
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
        (current.0 + c, current.1 + d),
        step + 1,
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
        (current.0 + e, current.1 + f),
        step + 1,
        wall,
        dp,
    );

    let count = (count_1 + count_2 + count_3) % MOD;

    *dp[step].entry(current).or_insert(0) = count;

    count
}
