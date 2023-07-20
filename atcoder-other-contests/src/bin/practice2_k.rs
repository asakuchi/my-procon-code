//!
//! セグメントツリー
//!

use proconio::input;

use ac_library_rs::ModInt998244353 as mint;
use ac_library_rs::{LazySegtree, MapMonoid, Monoid};

#[derive(Clone)]
struct Data {
    value: mint,
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
        Data {
            value: mint::from(0),
            size: 0,
        }
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
struct AdditiveData {
    b: mint,
    c: mint,
}

struct SumAdd;

impl MapMonoid for SumAdd {
    type M = Sum;
    /// 写像の型
    type F = AdditiveData;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        AdditiveData {
            b: mint::from(1),
            c: mint::from(0),
        }
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        Data {
            value: x.value * f.b + mint::from(x.size) * f.c,
            size: x.size,
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        // g(a) = g.b * a + g.c
        // f(a) = f.b * a + f.c
        // f(g(a)) = f(g.b * a + g.c)
        //         = f.b(g.b * a + g.c) + f.c
        //         = (f.b * g.b) * a + (f.b * g.c + f.c)
        AdditiveData {
            b: f.b * g.b,
            c: f.b * g.c + f.c,
        }
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    // 必ずsize=1
    let mut tree = LazySegtree::<SumAdd>::from(
        a.into_iter()
            .map(|x| Data {
                value: mint::from(x),
                size: 1,
            })
            .collect::<Vec<_>>(),
    );

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 0 {
            input! {
                l: usize,
                r: usize,
                b: mint,
                c: mint,
            }

            tree.apply_range(l, r, AdditiveData { b, c });
        } else {
            input! {
                l: usize,
                r: usize,
            }

            println!("{}", tree.prod(l, r).value);
        }
    }
}
