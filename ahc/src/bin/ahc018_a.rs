// for interactive
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
// println!("{}", x);
// stdout().flush().unwrap();

// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;
// use proconio::marker::Usize1;
// use rand::prelude::*;
// use rand::seq::SliceRandom;
// use rand::Rng;
// use std::collections::BinaryHeap;
// use std::time::Instant;

enum ExcavationResult {
    /// 指定したセルの岩盤が破壊できなかった
    NotBreak,
    /// 指定したセルの岩盤が破壊できた（家が存在するセルでまだ水が流れていないものがある）
    Broken,
    /// 指定したセルの岩盤が破壊できた（家が存在する全てのセルに水が流れている）
    Complete,
    /// 不正な出力
    Illegal,
}

fn main() {
    // let start = Instant::now();

    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        water_source: [(usize, usize); w],
        house: [(usize, usize); k],
    }

    let mut i = 0;

    // 正の得点を得る解法
    // 全部 5,000 のパワーで叩く

    for i in 0..n {
        for j in 0..n {
            let replay = query(i, j, 5_000, &mut source);

            match replay {
                ExcavationResult::Complete | ExcavationResult::Illegal => return,
                ExcavationResult::NotBreak => {}
                ExcavationResult::Broken => {}
            }
        }
    }
}

fn query<R: BufRead>(y: usize, x: usize, p: usize, source: &mut LineSource<R>) -> ExcavationResult {
    println!("{} {} {}", y, x, p);
    stdout().flush().unwrap();

    input! {
        from source,
        r: isize,
    }

    match r {
        0 => ExcavationResult::NotBreak,
        1 => ExcavationResult::Broken,
        2 => ExcavationResult::Complete,
        _ => ExcavationResult::Illegal,
    }
}

// fn initialize(
//     n: usize,
//     m: usize,
//     d: usize,
//     u_v_w: &mut Vec<(usize, usize, i32)>,
// ) -> Vec<Vec<usize>> {
// }

// fn to_output(plan_day: Vec<Vec<usize>>, m: usize, d: usize) -> Output {
//     let mut result = vec![0; m];

//     for day in 0..d {
//         for &load in &plan_day[day] {
//             result[load] = day + 1;
//         }
//     }

//     result
// }

// -------------------------------

// 山登り

// let mut rng = rand::thread_rng();

// let output = to_output(plan_day, m, d);

// let end = start.elapsed();

// let (score, _err, _) = compute_score(&input, &output);

// let end = start.elapsed();

// loop {
//     let end = start.elapsed();
//     if end.as_millis() >= 4_600 {
//         // eprintln!("timeup");

//         break;
//     }

//     let reply = query(x, &mut source);
// }

// println!("{}", text);
