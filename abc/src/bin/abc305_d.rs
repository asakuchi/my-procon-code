use proconio::input;
use superslice::Ext;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        l_r: [(usize, usize); q],
    }

    let mut list = vec![0; n + 1];

    for i in 1..n {
        list[i + 1] += list[i];

        if i % 2 == 0 {
            list[i + 1] += a[i] - a[i - 1];
        }
    }

    for &(l, r) in &l_r {
        let left = a.upper_bound(&l);
        let right = a.lower_bound(&r);

        let mut sleep = list[right + 1] - list[left];

        if left % 2 == 0 {
            // 寝てる
            sleep -= l - a[left - 1];
        } else {
            // 起きてる
        }

        if right % 2 == 0 {
            // 寝てる
            sleep -= a[right] - r;
        } else {
            // 起きてる
        }

        println!("{}", sleep);
    }
}
