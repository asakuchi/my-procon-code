use proconio::fastout;
use proconio::{input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                k: Usize1,
                x: usize,
            }

            a[k] = x;
        } else {
            input! {
                k: Usize1,
            }

            println!("{}", a[k]);
        }
    }
}
