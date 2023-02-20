use proconio::input;

use ::ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        n: u64,
    }

    let result = mint::from(10).pow(n) - mint::from(9).pow(n) * 2 + mint::from(8).pow(n);

    println!("{}", result);
}
