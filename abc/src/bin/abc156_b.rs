use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = Vec::new();

    let mut x = n;

    while x != 0 {
        result.push(x % k);
        x /= k;
    }

    println!("{}", result.len());
}
