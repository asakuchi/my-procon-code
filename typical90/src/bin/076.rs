// use proconio::fastout;
use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let total: usize = a.iter().sum();

    if total % 10 != 0 {
        println!("No");
        return;
    }

    let mut sum = vec![0; 2 * n + 1];

    for i in 0..2 * n {
        sum[i + 1] = a[i % n] + sum[i];
    }

    let mut s = 0;
    let mut t = 0;

    'search: loop {
        while sum[t] - sum[s] < total / 10 {
            t += 1;

            if t >= 2 * n {
                break 'search;
            }
        }

        if sum[t] - sum[s] == total / 10 {
            println!("Yes");
            return;
        }

        while sum[t] - sum[s] > total / 10 {
            s += 1;
        }

        if sum[t] - sum[s] == total / 10 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
