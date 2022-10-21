use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut i = 0;

    while i * 1_000 < n {
        i += 1;
    }

    println!("{}", i * 1_000 - n);
}
