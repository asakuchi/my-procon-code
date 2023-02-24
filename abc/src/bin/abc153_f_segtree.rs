//!
//! 遅延セグメントツリー
//!

use proconio::input;

use ac_library_rs::{LazySegtree, MapMonoid, Monoid};
use superslice::Ext;

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
    input! {
        n: usize,
        d: usize,
        a: usize,
        mut x_h: [(usize, usize); n],
    }

    x_h.sort();

    let mut tree = LazySegtree::<SumAdd>::from(vec![(0, 1); n]);

    let mut result = 0;

    for i in 0..n {
        let (x, mut h) = x_h[i];

        // 前の爆発で蓄積されたダメージ
        let prev_damage = tree.prod(i, i + 1).0;

        if h <= prev_damage {
            // 倒せる
            continue;
        }

        h -= prev_damage;

        // x + d で爆弾を使う
        let count = h / a + if h % a != 0 { 1 } else { 0 };
        result += count;

        // x + d で爆弾を使うと x + 2d まで届く
        let target = x_h.upper_bound_by_key(&(x + 2 * d), |&(target_x, _h)| target_x);
        tree.apply_range(i, target, a * count);
    }

    println!("{}", result);
}
