use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(isize, isize); n],
    }

    for (a, b) in a_b {
        println!("{}", a + b);
    }
}
