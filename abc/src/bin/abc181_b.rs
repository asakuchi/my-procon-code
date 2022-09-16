use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut result = 0;

    for (a, b) in a_b {
        result += (b - a + 1) * (a + b) / 2;
    }

    println!("{}", result);
}
