use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    for e in 1..=n - 1 {
        let f = n - e;

        let e_div = divisors(e);

        let f_div = divisors(f);

        result += e_div.len() * f_div.len();
    }

    println!("{}", result);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    {
        let mut i = 1;

        while i * i <= n {
            if n % i == 0 {
                list.push(i);

                if n / i != i {
                    list.push(n / i);
                }
            }

            i += 1;
        }
    }

    list.sort();

    list
}
