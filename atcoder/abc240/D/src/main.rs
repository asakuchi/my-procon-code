use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    // 番号 個数
    let mut stack: Vec<(isize, isize)> = Vec::new();

    let mut result = 0;

    for num in a {
        if let Some(tail) = stack.pop() {
            if tail.0 == num {
                let streek = tail.1 + 1;
                result += 1;

                stack.push((num, streek));
            } else {
                stack.push(tail);
                stack.push((num, 1));
                result += 1;
            }
        } else {
            stack.push((num, 1));
            result += 1;
        }

        // 連続していたら削除
        while let Some(tail) = stack.pop() {
            if tail.0 == tail.1 {
                result -= tail.1;
            } else {
                stack.push(tail);
                break;
            }
        }

        println!("{}", result);
    }
}
