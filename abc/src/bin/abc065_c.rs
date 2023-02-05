use proconio::input;

use ::ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if (n as isize - m as isize).abs() >= 2 {
        println!("0");
        return;
    }

    let mut result = mint::from(1);

    for i in 1..=n {
        result *= mint::from(i);
    }

    for i in 1..=m {
        result *= mint::from(i);
    }

    if n == m {
        // 数が同じなら犬と猿を入れ替えることができる
        result *= 2;
    }

    println!("{}", result);
}
