//!
//! Range Sum Query and Range Add Query (RSQ and RAQ)
//! 遅延評価セグメント木
//!
//! https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_G
//!

use ac_library_rs::{LazySegtree, MapMonoid, Monoid};

struct Sum;

impl Monoid for Sum {
    type S = (usize, usize);

    fn identity() -> Self::S {
        (0, 0)
    }

    fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
        (a + b, n + m)
    }
}

struct SumAdd;

impl MapMonoid for SumAdd {
    type M = Sum;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &(x, size): &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x + size * f, size)
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    let (n, q) = input_tuple();

    // 必ずsize=1
    let mut tree = LazySegtree::<SumAdd>::from(vec![(0, 1); n]);

    for _ in 0..q {
        let (com, s, t, x) = input_query();

        match com {
            0 => {
                // 1-index なので -1
                tree.apply_range(s - 1, t, x);
            }
            _ => {
                // 1-index なので -1
                println!("{}", tree.prod(s - 1, t).0);
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

fn input_query() -> (usize, usize, usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap_or("0").parse().unwrap_or(0);

    (n, m, l, k)
}
