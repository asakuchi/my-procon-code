use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    }

    let mut ok = 1_000_000.;
    let mut ng = 0.;

    // 実数で探索する場合の終了条件
    for _ in 0..100 {
        let mid = (ok + ng) / 2.;

        if f(a, b, c, mid) > 100. {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn f(a: f64, b: f64, c: f64, t: f64) -> f64 {
    a * t + b * (c * t * std::f64::consts::PI).sin()
}
