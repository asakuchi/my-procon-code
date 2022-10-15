use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", f(n));
}

fn f(x: usize) -> usize {
    if x == 0 {
        return 1;
    }

    x * f(x - 1)
}
