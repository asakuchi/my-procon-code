use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        m: usize,
        mut shop: [usize; n - 1],
        mut k: [usize; m],
    }

    shop.push(0);
    shop.push(d);
    shop.sort();

    let mut result = 0;

    for target in k {
        let mut ok: isize = n as isize;
        let mut ng = -1;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;

            let solve = target < shop[mid as usize];

            if solve {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        let score = (shop[ok as usize] - target).min(target - shop[(ok - 1) as usize]);

        result += score;
    }

    println!("{}", result);
}
