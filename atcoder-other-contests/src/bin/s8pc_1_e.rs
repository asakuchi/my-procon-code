use proconio::{input, marker::Usize1};

use ac_library_rs::ModInt1000000007;

use ModInt1000000007 as mint;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        mut c: [Usize1; q],
    }

    // 街0に戻る
    c.push(0);

    let mut s = vec![mint::from(0); n];

    for i in 1..n {
        s[i] = s[i - 1] + mint::from(a[i - 1]).pow(a[i] as u64);
    }

    let mut result = mint::from(0);

    // 初めは街1にいる
    let mut current = 0;

    for next in c {
        let bigger = current.max(next);
        let smaller = current.min(next);

        let diff = s[bigger] - s[smaller];

        result += diff;

        current = next;
    }

    println!("{}", result);
}
