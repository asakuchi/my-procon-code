use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // 常にmを大きくする
    let (n, m) = if n < m { (n, m) } else { (m, n) };

    // 自身を含む隣接しているマスの数だけ裏返す
    // -> 自身を含む隣接しているマスの数が偶数なら表、奇数なら裏
    // -> 自身を含む隣接しているマスの数が奇数の数を数える

    // 1 * 1
    if n == 1 && m == 1 {
        println!("1");
        return;
    }

    // 1 * 2
    if n == 1 && m == 2 {
        println!("0");
        return;
    }

    // 1 * 3以上
    if n == 1 && m >= 3 {
        println!("{}", m - 2);
        return;
    }

    // 2 * 2
    if n == 2 && m == 2 {
        println!("0");
        return;
    }

    // 2 * 3以上
    if n == 2 && m >= 3 {
        println!("0");
        return;
    }

    // n,m 共に3以上なら
    // 端以外が裏になる

    println!("{}", (n - 2) * (m - 2));

    // □□□□
    // □□□□
    // □□□□
    // □□□□
    // ---------
    // ■■□□
    // ■■□□
    // □□□□
    // □□□□

    // □□■□
    // □□■□
    // □□□□
    // □□□□

    // □■□■
    // □■□■
    // □□□□
    // □□□□

    // □■■□
    // □■■□
    // □□□□
    // □□□□
    // ---------
    // ■□■□
    // ■□■□
    // ■■□□
    // □□□□

    // □■□□
    // □■□□
    // □□■□
    // □□□□

    // □□■■
    // □□■■
    // □■□■
    // □□□□

    // □□□□
    // □□□□
    // □■■□
    // □□□□
    // ---------
    // □□□□
    // ■■□□
    // ■□■□
    // ■■□□

    // □□□□
    // □□■□
    // □■□□
    // □□■□

    // □□□□
    // □■□■
    // □□■■
    // □■□■

    // □□□□
    // □■■□
    // □□□□
    // □■■□
    // ---------
}
