//!
//! Range Minimum Query and Range Add Query (RMQ and RAQ)
//! 遅延評価セグメント木
//!
//! https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_H
//!

use ac_library_rs::{LazySegtree, MapMonoid, Min};

struct MinAdd;

impl MapMonoid for MinAdd {
    type M = Min<isize>;
    type F = isize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &Self::F) -> Self::F {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    let (n, q) = input_tuple();

    let mut tree = LazySegtree::<MinAdd>::from(vec![0; n]);

    for _ in 0..q {
        let (com, s, t, x) = input_query();

        match com {
            0 => {
                // 入力は閉区間で渡されるので +1
                tree.apply_range(s, t + 1, x);
            }
            _ => {
                // 入力は閉区間で渡されるので +1
                println!("{}", tree.prod(s, t + 1));
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

fn input_query() -> (usize, usize, usize, isize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let k: isize = iter.next().unwrap_or("0").parse().unwrap_or(0);

    (n, m, l, k)
}
