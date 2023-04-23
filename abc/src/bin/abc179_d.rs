use ac_library_rs::Monoid;
use proconio::input;

use ac_library_rs::ModInt998244353 as mint;
use ac_library_rs::Segtree;

pub struct Additive;

impl Monoid for Additive {
    type S = mint;
    fn identity() -> Self::S {
        mint::from(0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        l_r: [(usize, usize); k],
    }

    let mut tree = Segtree::<Additive>::new(n + 1);

    tree.set(1, mint::from(1));

    for i in 2..=n {
        let mut total = mint::from(0);

        for &(l, r) in &l_r {
            let left = if i > r { i - r } else { 0 };
            let right = if i > l { i - l } else { 0 };

            total += tree.prod(left, right + 1);
        }

        tree.set(i, total);
    }

    println!("{}", tree.get(n));
}
