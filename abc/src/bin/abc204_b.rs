use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = 0;

    for value in a {
        if value >= 10 {
            answer += value - 10;
        }
    }

    println!("{}", answer);
}
