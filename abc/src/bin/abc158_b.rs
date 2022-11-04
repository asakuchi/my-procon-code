use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let units = n / (a + b);
    let rest = n % (a + b);

    let result = a * units + if rest <= a { rest } else { a };

    println!("{}", result);
}
