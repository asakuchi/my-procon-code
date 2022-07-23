use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        c_y: [(usize, usize); m],
    }

    // dp[i][count]
    let mut dp = vec![vec![-1; 5_001]; 5_001];

    let mut map = HashMap::new();

    for &(c, y) in &c_y {
        map.entry(c).or_insert(y);
    }

    let result = rec(n, m, &x, &mut map, 0, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    x: &Vec<usize>,
    map: &HashMap<usize, usize>,
    i: usize,
    count: usize,
    dp: &mut Vec<Vec<isize>>,
) -> usize {
    if i == n {
        return 0;
    }

    if dp[i][count] != -1 {
        return dp[i][count] as usize;
    }

    let bonus;

    if let Some(&bonus_2) = map.get(&(count + 1)) {
        bonus = bonus_2;
    } else {
        bonus = 0;
    }

    // 表を出す
    let score_1 = rec(n, m, x, map, i + 1, count + 1, dp) + x[i] + bonus;

    // 裏を出す
    let score_2 = rec(n, m, x, map, i + 1, 0, dp);

    let score = score_1.max(score_2);

    dp[i][count] = score as isize;

    score
}
