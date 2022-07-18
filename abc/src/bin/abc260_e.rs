use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize); n],
    }

    let mut inverse = vec![Vec::new(); m + 1];

    for i in 0..n {
        inverse[ab[i].0].push(i);
        inverse[ab[i].1].push(i);
    }

    let mut count = vec![0; n];
    let mut result = vec![0; m + 3];
    let mut count_zero = n;

    {
        let mut i = 1;
        let mut j = 1;

        while i <= m {
            while j <= m && count_zero != 0 {
                for &x in inverse[j].iter() {
                    if count[x] == 0 {
                        count_zero -= 1;
                    }
                    count[x] += 1;
                }
                j += 1;
            }

            if count_zero != 0 {
                break;
            }

            result[j - i] += 1;
            result[m + 1 - i + 1] -= 1;

            for &x in inverse[i].iter() {
                count[x] -= 1;

                if count[x] == 0 {
                    count_zero += 1;
                }
            }

            i += 1;
        }
    }

    for i in 1..=m {
        result[i] += result[i - 1];
        println!("{}", result[i]);
    }
}
