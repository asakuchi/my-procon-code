use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n - 1],
    }

    let sum: usize = a.iter().sum();

    // n 科目で n * m 点以上取りたい
    // 最後のテストで満点とっても届かなければ不可能
    if m * n > sum + k {
        println!("-1");
        return;
    }

    if sum > m * n {
        println!("0");
        return;
    }

    println!("{}", m * n - sum);
}
