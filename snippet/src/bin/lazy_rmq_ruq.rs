//!
//! Range Minimum Query and Range Update Query (RMQ and RUQ)
//! 遅延評価セグメント木
//!
//! https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_F
//!

use ac_library_rs::{LazySegtree, MapMonoid, Min, Monoid};

const ID: usize = 1_000_000_000_000_000;

struct MinUpdate;

impl MapMonoid for MinUpdate {
    type M = Min<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        ID
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if f != ID {
            f
        } else {
            x
        }
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        if f == ID {
            g
        } else {
            f
        }
    }
}

fn main() {
    let (n, q) = input_tuple();

    let mut tree = LazySegtree::<MinUpdate>::from(vec![(1 << 31) - 1; n]);

    for _ in 0..q {
        let (com, s, t, x) = input_tuple_4();

        match com {
            0 => {
                tree.apply_range(s, t + 1, x);
            }
            _ => {
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

fn input_tuple_4() -> (usize, usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let o: usize = iter.next().unwrap().parse().unwrap();
    let p: usize = iter.next().unwrap_or("0").parse().unwrap();

    (n, m, o, p)
}
