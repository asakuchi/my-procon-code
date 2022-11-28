use ac_library_rs::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut ft = FenwickTree::new(n, 0usize);

    for i in 0..n {
        ft.add(i, a[i]);
    }

    for _ in 0..q {
        input! {t: usize}

        if t == 0 {
            input! {
                p: usize,
                x: usize,
            }

            ft.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }

            println!("{}", ft.accum(r) - ft.accum(l));
        }
    }
}
