//!
//! Range Sum Query(RSQ)
//! セグメント木
//!
//! https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=ja
//!

use ac_library_rs::{Additive, Segtree};

fn main() {
    let (n, q) = input_tuple();

    let mut tree = Segtree::<Additive<usize>>::new(n + 1);

    for _ in 0..q {
        let (com, x, y) = input_tuple_3();

        match com {
            0 => {
                // x は 1-index
                let past = tree.get(x - 1);

                tree.set(x - 1, past + y);
            }
            _ => {
                // x,y は 1-index
                println!("{}", tree.prod(x - 1, y));
            }
        }
    }
}

fn input_tuple() -> (usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_tuple_3() -> (usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();

    (n, m, l)
}
