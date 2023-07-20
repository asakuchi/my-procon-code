//!
//! Range Sum Query and Range Update Query (RSQ and RUQ)
//! 遅延評価セグメント木
//!
//! https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_I
//!

use ac_library_rs::{LazySegtree, MapMonoid, Monoid};

#[derive(Clone)]
struct Data {
    value: isize,
    size: usize,
}

struct Sum;

impl Monoid for Sum {
    ///
    /// モノイドの型
    ///
    type S = Data;

    ///
    /// 単位元
    ///
    fn identity() -> Self::S {
        Data { value: 0, size: 0 }
    }

    ///
    /// 二項演算
    ///
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Data {
            value: a.value + b.value,
            size: a.size + b.size,
        }
    }
}

#[derive(Clone)]
enum MappingType {
    Value(isize),
    ID,
}

struct SumUpdate;

impl MapMonoid for SumUpdate {
    type M = Sum;
    /// 写像の型
    type F = MappingType;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        MappingType::ID
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let MappingType::Value(value) = f {
            Data {
                value: x.size as isize * value,
                size: x.size,
            }
        } else {
            // f が ID ならそのまま x を返す
            x.clone()
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if let MappingType::Value(_) = f {
            // 後からの操作で上書き
            f.clone()
        } else {
            // f が ID ならそのまま g を返す
            g.clone()
        }
    }
}

fn main() {
    let (n, q) = input_tuple();

    // 必ずsize=1
    let mut tree = LazySegtree::<SumUpdate>::from(vec![Data { value: 0, size: 1 }; n]);

    for _ in 0..q {
        let (com, s, t, x) = input_query();

        match com {
            0 => {
                // 入力は閉区間で渡されるので +1
                tree.apply_range(s, t + 1, MappingType::Value(x));
            }
            _ => {
                // 入力は閉区間で渡されるので +1
                println!("{}", tree.prod(s, t + 1).value);
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
