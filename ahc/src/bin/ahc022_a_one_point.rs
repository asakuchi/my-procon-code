// 1点高温法

use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        l: usize,
        n: usize,
        _s: usize,
        y_x: [(usize, usize); n],
    }

    let (y_0, x_0) = y_x[0];

    // 出口セル0と各出口セルの差
    let mut distances = vec![(0, 0); n];

    for j in 0..n {
        let (y_j, x_j) = y_x[j];
        distances[j] = (y_0 as isize - y_j as isize, x_0 as isize - x_j as isize);
    }

    //
    // 配置
    //

    let mut p = vec![vec![0; l]; l];
    p[y_0][x_0] = 1_000;

    for line in p.iter() {
        println!("{}", line.iter().format(" "));
    }

    stdout().flush().unwrap();

    //
    // 計測
    //

    // i 番目のワームホールが E[i]番目の出口セルに対応する
    let mut result = vec![0; n];

    // i番目の出口セルは使用済み
    let mut used = vec![false; n];

    for i in 0..n {
        for j in 0..n {
            if used[j] {
                continue;
            }

            let distance = distances[j];

            println!("{} {} {}", i, distance.0, distance.1);
            stdout().flush().unwrap();

            input! {
                from &mut source,
                m: usize,
            }

            println!("# m: {}", m);

            if m > 500 {
                result[i] = j;
                used[j] = true;
            }
        }
    }

    println!("-1 -1 -1");
    stdout().flush().unwrap();

    for i in 0..n {
        println!("{}", result[i]);
    }

    stdout().flush().unwrap();
}
