use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut set = std::collections::HashSet::new();

    let mut a = 2;

    while a * a <= n {
        let mut num = a * a;

        while num <= n {
            set.insert(num);

            num *= a;
        }

        a += 1;
    }

    println!("{}", n - set.len());
}
