//!
//! 二分探索
//!

fn main() {
    let mut ok: isize = 1 << 60;
    let mut ng = -1;

    // 実数で探索する場合の終了条件
    // for _ in 0..100 {

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || true;

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
