use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for price in 1..=1_000_000 {
        let a_candi = price * 8 / 100;
        let b_candi = price * 10 / 100;

        if a == a_candi && b == b_candi {
            println!("{}", price);
            return;
        }
    }

    println!("-1");
}
