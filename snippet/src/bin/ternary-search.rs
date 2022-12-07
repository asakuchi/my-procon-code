//!
//! 三分探索
//!

fn main() {
    let mut low = 0;
    let mut high = 1_000_000_000_000_000_000;

    // 実数で探索する場合の終了条件
    // for _ in 0..100 {

    while high - low > 3 {
        let c1 = (low * 2 + high) / 3;
        let c2 = (low + high * 2) / 3;

        if calc(c1) > calc(c2) {
            low = c1;
        } else {
            high = c2
        }
    }

    println!("{}", low);
}

fn calc(time: usize) -> f64 {
    todo!();
}
