use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        mut lrv: [(usize, usize, isize); q],
    }

    let mut b = vec![0; n + 1];

    for i in 1..n {
        b[i] += a[i] - a[i - 1];
    }

    let mut sum: isize = b.iter().map(|x| x.abs()).sum();

    for &(l, r, v) in lrv.iter() {
        let before = (b[l - 1]).abs() + b[r].abs();

        if l >= 2 {
            b[l - 1] += v;
        }

        if r <= n - 1 {
            b[r] -= v;
        }

        let after = (b[l - 1]).abs() + b[r].abs();

        sum += after - before;

        println!("{}", sum);
    }
}
