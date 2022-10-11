use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize,
    }

    println!("{}", n.min(n % k).min((n % k - k).abs()));
}
