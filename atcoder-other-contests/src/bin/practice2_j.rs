use ac_library_rs::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    }

    // usize はダメ
    let mut tree = Segtree::<Max<_>>::from(a);

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                x: Usize1,
                v: isize,
            }

            tree.set(x, v);
        } else if query == 2 {
            input! {
                l: Usize1,
                r: Usize1,
            }

            println!("{}", tree.prod(l, r + 1));
        } else {
            input! {
                x: Usize1,
                v: isize,
            }

            println!("{}", tree.max_right(x, |value| value < &v) + 1);
        }
    }
}
