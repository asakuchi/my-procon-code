use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x_y: [(isize, isize); n],
        p_q: [(isize, isize); m],
    }

    let mut points = Vec::new();

    for &(p, q) in &p_q {
        points.push((p, q));
    }

    for &(x, y) in &x_y {
        points.push((x, y));
    }

    // 原点
    points.push((0, 0));
    // 原点
    let current = n + m;

    let speed = 1.;

    let mut result = 1_000_000_000_000_000.;

    let mut dp = vec![vec![None; 1 << (n + m + 1)]; n + m + 1];

    for next in 0..n + m {
        let current_point = points[current];
        let next_point = points[next];

        let cost = ((current_point.0 - next_point.0).pow(2) as f64
            + (current_point.1 - next_point.1).pow(2) as f64)
            .sqrt()
            / speed;

        let score_1 = rec(n, m, &points, 1 << next, next, &mut dp) + cost;

        if result > score_1 {
            result = score_1;
        }
    }

    println!("{}", result);
}

fn rec(
    n: usize,
    m: usize,
    points: &Vec<(isize, isize)>,
    visited: usize,
    current: usize,
    dp: &mut Vec<Vec<Option<f64>>>,
) -> f64 {
    // println!("current {} visited {:04b}", current, visited);

    // n + m を原点とする
    if current == n + m {
        // println!("visited {:04b} visited >> m {:04b}", visited, visited >> m);

        // 原点も含めてn個の街に到達済みか
        if visited >> m == (1 << n + 1) - 1 {
            return 0.;
        } else {
            return 1_000_000_000_000_000.;
        }
    }

    if let Some(value) = dp[current][visited] {
        return value;
    }

    let mut result = 1_000_000_000_000_000.;

    let mut speed = 1.;

    for treasure in 0..m {
        // println!(
        //     "speed check visited {:04b} treasure {} {:04b}",
        //     visited,
        //     treasure,
        //     1 << treasure
        // );

        if visited & 1 << treasure > 0 {
            // println!("speed up");
            speed *= 2.;
        }
    }

    // println!("speed {} ", speed);

    for next in 0..n + m + 1 {
        if visited & 1 << next == 0 {
            let current_point = points[current];
            let next_point = points[next];

            let cost = ((current_point.0 - next_point.0).pow(2) as f64
                + (current_point.1 - next_point.1).pow(2) as f64)
                .sqrt()
                / speed;

            let score_1 = rec(n, m, points, visited | 1 << next, next, dp) + cost;

            if result > score_1 {
                result = score_1;
            }
        }
    }

    dp[current][visited] = Some(result);

    result
}
