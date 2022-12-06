//!
//! Range Sum Query and Range Update Query (RSQ and RUQ)
//! 遅延評価セグメント木
//!
//! https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_I
//!

use ac_library_rs::{LazySegtree, MapMonoid, Monoid};

const ID: isize = 1_000_000_000_000_000;

struct Sum;

impl Monoid for Sum {
    type S = (isize, usize);

    fn identity() -> Self::S {
        (0, 0)
    }

    fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
        (a + b, n + m)
    }
}

struct SumUpdate;

impl MapMonoid for SumUpdate {
    type M = Sum;
    type F = isize;

    fn identity_map() -> Self::F {
        ID
    }

    fn mapping(&f: &Self::F, &(x, size): &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if f != ID {
            (size as isize * f, size)
        } else {
            (x, size)
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

    // 必ずsize=1
    let mut tree = LazySegtree::<SumUpdate>::from(vec![(0, 1); n]);

    for _ in 0..q {
        let (com, s, t, x) = input_query();

        match com {
            0 => {
                // 入力は閉区間で渡されるので +1
                tree.apply_range(s, t + 1, x);
            }
            _ => {
                // 入力は閉区間で渡されるので +1
                println!("{}", tree.prod(s, t + 1).0);
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
