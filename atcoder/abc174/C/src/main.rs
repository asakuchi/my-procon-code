use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut num = 7;

    for i in 1..=k {
        if num % k == 0 {
            println!("{}", i);
            return;
        }

        num *= 10;
        num %= k;
        num += 7;
        num %= k;
    }

    println!("-1");
}
