use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }

    let mut result: usize = 0;

    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if a[i] % p * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", result);
}
