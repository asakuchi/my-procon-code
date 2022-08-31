use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let list = divisors(k);

    let mut result = 0usize;

    let n = list.len();

    for i in 0..n {
        let a = list[i];

        for j in i..n {
            let b = list[j];

            for l in j..n {
                let c = list[l];

                if a * b * c == k {
                    result += 1;
                }

                if a * b * c > k {
                    break;
                }
            }
        }
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
