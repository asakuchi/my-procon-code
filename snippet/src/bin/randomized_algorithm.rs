//!
//! 乱択アルゴリズム
//!

use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

fn main() {
    let n = 10;

    let start = Instant::now();

    let mut rng = rand::thread_rng();

    loop {
        // 時間制限
        let end = start.elapsed();
        if end.as_millis() >= 1800 {
            break;
        }

        // 探索
        // メインロジック
        {
            let mut index_list: Vec<_> = (0..n).collect();

            // Vecを適当な順番に
            index_list.shuffle(&mut rng);

            // 0 以上 n 未満の乱数
            let target = rng.gen_range(0, n);
        }
    }

    // 見つからなかった
    println!("-1");
}
