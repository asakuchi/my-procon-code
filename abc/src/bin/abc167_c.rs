use proconio::fastout;
use proconio::input;

const INF: usize = 1_000_000_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        ca: [(usize, [usize; m]); n],
    }

    let mut min_cost = INF;

    for mask in 0..1 << n {
        let mut cost = 0;
        let mut understanding = vec![0; m];

        for i in 0..n {
            let (c, a) = &ca[i];

            if mask & 1 << i > 0 {
                cost += c;

                for j in 0..m {
                    understanding[j] += a[j];
                }
            }
        }

        if understanding.iter().all(|&value| value >= x) {
            min_cost = min_cost.min(cost);
        }
    }

    if min_cost == INF {
        println!("-1");
    } else {
        println!("{}", min_cost);
    }
}
