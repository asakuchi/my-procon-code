use proconio::fastout;
use proconio::input;
use proconio::marker::Isize1;

const INF: isize = 100_100_100_100;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: Isize1,
        y: Isize1,
    }

    // 始点から各頂点までの最短コスト
    let mut costs = vec![vec![INF; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            let cost_not_using_xy = j as isize - i as isize;
            let cost_using_xy = (x - i as isize).abs() + 1 + (j as isize - y).abs();

            costs[i][j] = cost_not_using_xy.min(cost_using_xy) as isize;
        }
    }

    let mut result = std::collections::HashMap::new();

    for i in 0..n {
        for j in i + 1..n {
            *result.entry(costs[i][j]).or_insert(0) += 1;
        }
    }

    for k in 1..n {
        println!("{}", result.entry(k as isize).or_insert(0));
    }
}
