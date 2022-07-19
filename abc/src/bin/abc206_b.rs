use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut money = 0;

    for i in 1..100_000 {
        money += i;

        if money >= n {
            println!("{}", i);
            return;
        }
    }
}
