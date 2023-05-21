use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
    }

    p.push(0);
    p.sort();

    let n = n + 1;

    // 2本投げた時の点数
    let mut q = Vec::new();

    for i in 0..n {
        for j in 0..n {
            q.push(p[i] + p[j]);
        }
    }

    q.sort();

    let mut result = 0_usize;

    for i in 0..q.len() {
        if q[i] > m {
            break;
        }

        // q[i] + q[j] <= m
        // q[j] <= m - q[i]

        let j = q.upper_bound(&(m - q[i]));

        if j != 0 {
            result = result.max(q[i] + q[j - 1]);
        }
    }

    println!("{}", result);
}
