use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut s = vec![0; n - 1];

    for i in 0..n - 1 {
        if a[i + 1] > a[i] {
            s[i] = 1;
        } else {
            s[i] = 0;
        }
    }

    let mut sum = 0;

    for i in 0..k - 1 {
        sum += s[i];
    }

    let mut result = 0;

    if sum == k - 1 {
        result += 1;
    }

    for i in k..n {
        sum -= s[i - k];
        sum += s[i - 1];

        if sum == k - 1 {
            result += 1;
        }
    }

    println!("{}", result);
}
