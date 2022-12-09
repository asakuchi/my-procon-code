use proconio::input;

const INF: usize = 1_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
    }

    let mut total_cost = 0;

    for i in 0..n {
        for j in i + 1..n {
            total_cost += a[i][j];
        }
    }

    for k in 0..n {
        for i in 0..n {
            if i == k {
                continue;
            }

            for j in i + 1..n {
                if j == k {
                    continue;
                }

                if a[i][j] == INF || a[i][k] == INF || a[k][j] == INF {
                    continue;
                }

                if a[i][j] > a[i][k] + a[k][j] {
                    println!("-1");
                    return;
                }

                if a[i][j] == a[i][k] + a[k][j] {
                    // 迂回して到達するコストと、直接移動するコストが同じなら
                    // 直接移動する道路は不要
                    total_cost -= a[i][j];

                    a[i][j] = INF;
                }
            }
        }
    }

    println!("{}", total_cost);
}
