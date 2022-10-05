use proconio::input;

fn main() {
    input! {
        x: u128
    }

    let mut yen = 100;

    let mut year = 0;

    while yen < x {
        yen = yen + yen / 100;
        year += 1;
    }

    println!("{}", year);
}
