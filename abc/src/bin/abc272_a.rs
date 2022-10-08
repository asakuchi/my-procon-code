use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();

    println!("{}", sum);
}
