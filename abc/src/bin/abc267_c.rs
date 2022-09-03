use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut sum = vec![0; n];

    for i in 0..m {
        sum[0] += a[i];
    }

    for i in 1..n - m + 1 {
        sum[i] = sum[i - 1];

        sum[i] += a[i + m - 1];
        sum[i] -= a[i - 1];
    }

    let mut prev = 0;

    for i in 0..m {
        prev += (i as isize + 1) * a[i];
    }

    let mut result = prev;

    for i in 1..n - m + 1 {
        prev += a[i + m - 1] * m as isize;
        prev -= sum[i - 1];

        result = result.max(prev);
    }

    println!("{}", result);
}
