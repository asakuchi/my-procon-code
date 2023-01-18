use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        k: Usize1,
    }

    // 5_000 兆日
    // 5 * 10^15 日

    // s の 2以上の数値を探す
    for i in 0..s.len() {
        let x: usize = s[i].to_string().parse().unwrap();

        if x > 1 {
            if i > k {
                println!("1");
            } else {
                println!("{}", x);
            }

            return;
        }
    }

    println!("1");
}
