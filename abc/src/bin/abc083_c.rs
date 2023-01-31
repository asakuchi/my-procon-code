use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut result = 0;

    let mut a = x;

    while a <= y {
        a *= 2;
        result += 1;
    }

    println!("{}", result);
}
