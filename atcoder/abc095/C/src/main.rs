use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
        x: i128,
        y: i128,
    }

    let mut result = std::i128::MAX;

    let max_count = std::cmp::max(x, y);

    for i in (0..=max_count * 2).step_by(2) {
        let req_x = std::cmp::max(x - i / 2, 0);
        let req_y = std::cmp::max(y - i / 2, 0);

        let price = a * req_x + b * req_y + c * i;

        result = std::cmp::min(result, price);
    }

    println!("{}", result);
}
