use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut result = 0;
    let mut hole = 1;

    while hole < b {
        result += 1;

        hole += a - 1;
    }

    println!("{}", result);
}
