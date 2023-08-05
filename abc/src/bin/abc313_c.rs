use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sum = 0;

    for i in 0..n {
        sum += a[i];
    }

    let lower = sum / n;
    let upper = sum / n + 1;

    let mut up_action = 0;
    let mut down_action = 0;

    for i in 0..n {
        if a[i] <= lower {
            up_action += lower - a[i];
        } else {
            down_action += a[i] - upper;
        }
    }

    println!("{}", up_action.max(down_action));
}
