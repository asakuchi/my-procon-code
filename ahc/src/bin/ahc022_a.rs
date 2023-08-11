use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

///
/// cargo equip --exclude-atcoder-crates --minify libs --remove docs --remove comments --bin ahc021_a | pbcopy
///

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        l: usize,
        n: usize,
        _s: usize,
        _y_x: [(usize, usize); n],
    }

    // 配置
    for _ in 0..l {
        let line = (0..l).map(|_| 0).format(" ");
        println!("{}", line);
    }

    stdout().flush().unwrap();

    // 計測
    println!("-1 -1 -1");
    stdout().flush().unwrap();

    for _ in 0..n {
        println!("{}", 0);
    }

    stdout().flush().unwrap();
}
