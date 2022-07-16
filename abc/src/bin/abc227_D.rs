use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    // P * K <= sum

    a.sort();

    let mut ok = 0isize;
    let mut ng = (1 << 60) / k as isize;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let sum: usize = a.iter().map(|&x| x.min(mid as usize)).sum();

        // println!("mid:{} k:{} mid*k:{} sum:{}", mid, k, mid as usize * k, sum);

        let solve = || mid as usize * k <= sum;

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
