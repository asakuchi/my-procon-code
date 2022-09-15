use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 2;
    let mut max_gcd = 0;

    for divisor in 2..=1000 {
        let mut gcd_degree = 0;

        for i in 0..n {
            if a[i] % divisor == 0 {
                gcd_degree += 1;
            }
        }

        if gcd_degree >= max_gcd {
            max_gcd = gcd_degree;
            result = divisor;
        }
    }

    println!("{}", result);
}
