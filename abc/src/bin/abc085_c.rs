use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }

    for i in 0..=n {
        for j in 0..=n - i {
            let k = n - i - j;

            if i * 10_000 + j * 5_000 + k * 1_000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
