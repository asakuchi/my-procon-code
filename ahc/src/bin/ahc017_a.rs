use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        u_v_w: [(Usize1, Usize1, usize); m],
        x_y: [(usize, usize); n],
    }

    // とりあえずスコアを獲得する提出
    let mut result = Vec::new();

    for day in 1..=d {
        for _ in 0..k {
            result.push(day);
        }
    }

    // println!("{:?}", &result[..m]);

    let text = &result[..m].iter().format(" ");

    println!("{}", text);
}
