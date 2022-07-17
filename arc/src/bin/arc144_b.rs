use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        list: [isize; n],
    }

    let limit = Integer::div_ceil(&list.iter().sum::<isize>(), &(n as isize));

    let mut ng = limit as isize + 100;
    let mut ok = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let solve = || {
            let mut x_count = 0;
            let mut y_count = 0;

            for &value in &list {
                if value < mid {
                    // 小さい
                    x_count += Integer::div_ceil(&(mid - value), &a);
                } else if value > mid {
                    // 大きい
                    y_count += (value - mid) / b;
                }
            }

            x_count <= y_count
        };

        if solve() {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
