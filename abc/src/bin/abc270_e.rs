use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ok: isize = 1_000_000_000_000;
    let mut ng = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || {
            let mut total = 0;

            for i in 0..n {
                total += if (a[i] as isize) < mid {
                    a[i] as isize
                } else {
                    mid
                };
            }

            total >= k as isize
        };

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let mut rest = k;

    let mut result = vec![0; n];

    // k - 1 周分を計算
    for i in 0..n {
        if a[i] < ok as usize - 1 {
            result[i] = 0;
            rest -= a[i];
        } else {
            result[i] = a[i] - ok as usize + 1;
            rest -= ok as usize - 1;
        }
    }

    // 1周未満
    for i in 0..n {
        if rest == 0 {
            break;
        }

        if result[i] > 0 {
            result[i] -= 1;
            rest -= 1;
        }
    }

    let result = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}
