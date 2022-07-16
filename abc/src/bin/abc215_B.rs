use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..=100 {
        if 2usize.pow(i) > n {
            println!("{}", i - 1);
            return;
        }
    }
}
