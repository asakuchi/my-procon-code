use ac_library_rs::{LazySegtree, MapMonoid, ModInt998244353, Monoid};

use proconio::input;

use ModInt998244353 as mint;

struct Sum;

impl Monoid for Sum {
    type S = (mint, mint);

    fn identity() -> Self::S {
        (0.into(), 0.into())
    }

    fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
        (a + b, n + m)
    }
}

struct SumAdd;

impl MapMonoid for SumAdd {
    type M = Sum;
    type F = mint;

    fn identity_map() -> Self::F {
        0.into()
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
        a: [usize; n - 1],
    }

    // dp
    // マスiからサイコロを投げる回数の期待値
    // let mut tree = LazySegmentTreeRsq::new(2 * n + 1);
    let mut tree = LazySegtree::<SumAdd>::from(vec![(0.into(), 1.into()); 2 * n + 1]);

    for i in (0..n - 1).rev() {
        // let dp_ij = tree.query(i, i + a[i] + 1);
        let dp_ij = tree.prod(i, i + a[i] + 1).0;
        // tree.add(i, i + 1, (dp_ij + mint::new(a[i] + 1)) / mint::new(a[i]));
        tree.apply_range(i, i + 1, (dp_ij + mint::new(a[i] + 1)) / mint::new(a[i]));
    }

    println!("{}", tree.prod(0, 1).0);
}
