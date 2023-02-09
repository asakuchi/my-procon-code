use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut total = 0;

    for i in 1..1_000_000_000 {
        total += i;

        if total >= x {
            println!("{}", i);
            return;
        }
    }
}
