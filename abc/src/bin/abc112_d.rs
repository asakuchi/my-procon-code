use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let list = divisors(m);

    for &num in list.iter().rev() {
        if num * n > m {
            continue;
        }

        println!("{}", num);
        return;
    }
}

fn divisors(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    {
        let mut i = 1;

        while i * i <= n {
            if n % i == 0 {
                list.push(i);
                list.push(n / i);
            }

            i += 1;
        }
    }

    list.sort();

    list
}
