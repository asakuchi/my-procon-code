use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut list = vec![0; n + 1];

    list[0] = 2_usize;
    list[1] = 1;

    for i in 2..=n {
        list[i] = list[i - 1] + list[i - 2];
    }

    println!("{}", list[n]);
}
