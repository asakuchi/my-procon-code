use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut s = vec![Vec::with_capacity(n); m];

    for i in 0..m {
        input! {
            k:usize,
            s_inner: [usize;k],
        }

        s[i] = s_inner.iter().map(|x| x - 1).collect();
    }

    input! {
        p: [usize; m],
    }

    let mut result = 0;

    for bit in 0..(1 << n) {
        let mut all_on = true;
        for j in 0..m {
            let on_count = s[j]
                .iter()
                .filter(|&&x| bit & (1 << x) > 0)
                .collect::<Vec<_>>()
                .len();

            if on_count % 2 != p[j] {
                all_on = false;
            }
        }
        if all_on {
            result += 1;
        }
    }

    println!("{}", result);
}
