use ac_library_rs::ModInt998244353;
use proconio::input;

fn main() {
    input! {
        a: ModInt998244353,
        b: ModInt998244353,
        c: ModInt998244353,
        d: ModInt998244353,
        e: ModInt998244353,
        f: ModInt998244353,
    }

    println!("{}", a * b * c - d * e * f);
}
