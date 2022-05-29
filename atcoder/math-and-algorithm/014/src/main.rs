use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let list = prime_factorize(n);

    let result = list
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}

fn prime_factorize(n: usize) -> Vec<usize> {
    let mut current = n;

    let mut list = Vec::new();

    {
        let mut i = 2;

        while i * i <= n {
            while current % i == 0 {
                list.push(i);
                current /= i;
            }

            if current == 1 {
                break;
            }

            i += 1;
        }

        if current != 1 {
            list.push(current);
        }
    }

    list
}
