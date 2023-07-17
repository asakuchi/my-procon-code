use proconio::{input, marker::Usize1};

use ac_library_rs::{LazySegtree, MapMonoid, Monoid};

struct Xor;

impl Monoid for Xor {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
        a ^ b
    }
}

struct XorAdd;

impl MapMonoid for XorAdd {
    type M = Xor;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        x ^ f
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f ^ g
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut tree = LazySegtree::<XorAdd>::from(a);

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: Usize1,
                y: usize,
            }

            tree.apply(x, y);
        } else {
            input! {
                x: Usize1,
                y: Usize1,
            }

            println!("{}", tree.prod(x, y + 1));
        }
    }
}
