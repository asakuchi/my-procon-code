use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u128; n],
    }

    for &value in &a {
        if value == 0 {
            println!("0");
            return;
        }
    }

    let mut result = 1;

    for value in a {
        result *= value;

        if result > 1_000_000_000_000_000_000 {
            println!("-1");
            return;
        }
    }

    if result > 1_000_000_000_000_000_000 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
