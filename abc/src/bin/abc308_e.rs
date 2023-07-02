use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut x_accum = vec![0; 3];

    for i in 0..n {
        if s[i] == 'X' {
            x_accum[a[i]] += 1;
        }
    }

    let mut m_accum = vec![0; 3];

    let mut result = 0_usize;

    for j in 0..n {
        if s[j] == 'X' {
            x_accum[a[j]] -= 1;
        } else if s[j] == 'M' {
            m_accum[a[j]] += 1;
        } else {
            for i in 0..=2 {
                for k in 0..=2 {
                    result += mex(i, a[j], k) * m_accum[i] * x_accum[k];
                }
            }
        }
    }

    println!("{}", result);
}

fn mex(a: usize, b: usize, c: usize) -> usize {
    for i in 0..=3 {
        if i != a && i != b && i != c {
            return i;
        }
    }

    100
}
