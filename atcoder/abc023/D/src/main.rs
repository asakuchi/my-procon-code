use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hs: [(isize, isize); n],
    }

    let mut ok: isize = 1_000_000_000_000_000;
    let mut ng = -1;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        hs.sort_by_key(|&(h, s)| (mid - h) / s);

        let mut solve = true;

        for i in 0..n {
            let (h, s) = hs[i];

            if mid < h {
                solve = false;
                break;
            }

            if mid < h + s * i as isize {
                solve = false;
                break;
            }
        }

        if solve {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
