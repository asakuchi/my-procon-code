use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    for i in 1..=n {
        let mut k = i;

        {
            let mut d = 2;

            while d * d <= k {
                while k % (d * d) == 0 {
                    k /= d * d;
                }

                d += 1;
            }
        }

        {
            let mut d = 1;

            while k * d * d <= n {
                result += 1;

                d += 1;
            }
        }
    }

    println!("{}", result);
}
