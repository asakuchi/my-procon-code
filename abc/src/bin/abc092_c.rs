use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut list = Vec::with_capacity(n);
    list.push(0);
    list.append(&mut a);
    list.push(0);

    let mut total = 0;

    for i in 1..n + 2 {
        total += (list[i] - list[i - 1]).abs();
    }

    for i in 1..=n {
        let mut score = total;

        // i-1 => i => i+1 を削除
        score -= (list[i] - list[i - 1]).abs() + (list[i + 1] - list[i]).abs();
        // i-1 => i+1 を追加
        score += (list[i + 1] - list[i - 1]).abs();

        println!("{}", score);
    }
}
