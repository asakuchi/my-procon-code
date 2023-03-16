use proconio::input;

fn main() {
    input! {
        x_1: isize,
        y_1: isize,
        x_2: isize,
        y_2: isize,
    }

    let a = y_1 - y_2;
    let b = x_1 - x_2;

    let x_3 = x_2 + a;
    let y_3 = y_2 - b;

    let x_4 = x_1 + a;
    let y_4 = y_1 - b;

    println!("{} {} {} {}", x_3, y_3, x_4, y_4);
}
