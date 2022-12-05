//!
//! Range Minimum Query(RMQ)
//! セグメント木
//!
//! https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A&lang=ja
//!

use ac_library_rs::{Min, Segtree};

fn main() {
    let (n, q) = input_nq();

    let mut tree = Segtree::<Min<usize>>::new(n);

    for i in 0..n {
        // 2^31 - 1 で初期化
        tree.set(i, std::i32::MAX as usize);
    }

    for _ in 0..q {
        let (com, x, y) = input_com_xy();

        match com {
            0 => {
                tree.set(x, y);
            }
            _ => {
                // 入力は閉区間で渡されるので +1
                println!("{}", tree.prod(x, y + 1));
            }
        }
    }
}

fn input_nq() -> (usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    (n, q)
}

fn input_com_xy() -> (usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let com: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let y: usize = iter.next().unwrap().parse().unwrap();

    (com, x, y)
}
