use std::io;

const INF: usize = 1_000_000_000;

fn main() {
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let e: usize = iter.next().unwrap().parse().unwrap();

    // ------------------------------------
    // タプルのベクタ

    let mut s_t_d: Vec<(usize, usize, usize)> = Vec::with_capacity(e);

    for _ in 0..e {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s: usize = iter.next().unwrap().parse().unwrap();
        let t: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t, d));
    }

    // ------------------------------------

    let mut list = vec![Vec::new(); v];

    for &(s, t, d) in &s_t_d {
        list[s].push((t, d));
    }

    let mut result = INF;

    for i in 0..v {
        let mut dp = vec![vec![-1; 1 << v]; v];
        let score = dfs(v, &list, i, i, 0, &mut dp);

        result = result.min(score);
    }

    if result != INF {
        println!("{}", result)
    } else {
        println!("-1");
    }
}

fn dfs(
    v: usize,
    list: &Vec<Vec<(usize, usize)>>,
    goal: usize,
    current: usize,
    visited: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if dp[current][visited] != -1 {
        return dp[current][visited] as usize;
    }

    if visited == (1 << v) - 1 && current == goal {
        dp[current][visited] = 0;
        return 0;
    }

    let mut result = INF;

    for &(next, cost) in list[current].iter() {
        if visited & 1 << next > 0 {
            continue;
        }

        let score = dfs(v, list, goal, next, visited | 1 << next, dp) + cost;

        result = result.min(score);
    }

    dp[current][visited] = result as isize;

    result
}
