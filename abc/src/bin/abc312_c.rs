use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_org: [isize; n],
        mut b_org: [isize; m],
    }

    let mut a = vec![0];
    let mut b = vec![0];

    a.append(&mut a_org);
    b.append(&mut b_org);

    a.push(1_000_000_001);
    b.push(1_000_000_001);

    a.sort();
    b.sort();

    // println!("a {:?}", a);
    // println!("b {:?}", b);

    let n = n + 2;
    let m = m + 2;

    let mut ok: isize = 1_000_000_002;
    let mut ng = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || {
            // 売っても良い人数
            let i = a.lower_bound(&(mid + 1));

            let can_sell = i - 1;

            // 買っても良い人数
            let j = b.lower_bound(&mid);

            let can_buy = m - j - 1;

            // println!("mid {} {} {}", mid, can_sell, can_buy);

            can_sell >= can_buy
        };

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
