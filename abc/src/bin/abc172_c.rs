use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut a_total = vec![0; n + 1];
    let mut b_total = vec![0; m + 1];

    for i in 0..n {
        a_total[i + 1] = a_total[i] + a[i];
    }

    for i in 0..m {
        b_total[i + 1] = b_total[i] + b[i];
    }

    let mut result = 0;

    let mut b_index = m as isize;

    for a_index in 0..n + 1 {
        while b_index >= 0 && k < a_total[a_index] + b_total[b_index as usize] {
            b_index -= 1;
        }

        if b_index >= 0 {
            result = result.max(a_index + b_index as usize);
        }
    }

    println!("{}", result);
}
