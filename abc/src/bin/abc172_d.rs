use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut result = 0;

    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            result += j;
        }
    }

    println!("{}", result);
}
