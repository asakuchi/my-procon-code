use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let every = k / n;
    let mut rest = k % n;

    // （入力の添え字i人目, 国民番号, お菓子の個数）
    let mut list: Vec<_> = a.iter().enumerate().map(|(a, &b)| (a, b, every)).collect();

    // 入力の国民番号でソート
    list.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 0..n {
        if rest > 0 {
            list[i].2 += 1;
            rest -= 1;
        } else {
            break;
        }
    }

    // 入力の添え字でソート
    list.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..n {
        println!("{}", list[i].2);
    }
}
