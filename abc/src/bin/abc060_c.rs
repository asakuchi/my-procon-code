use proconio::input;

fn main() {
    input! {
        n: usize,
        water: usize,
        mut t: [usize; n],
    }

    // 番兵
    t.push(1_000_000_000_000);

    let n = n + 1;

    let mut result = 0;

    for i in 1..n {
        if t[i] - t[i - 1] < water {
            result += t[i] - t[i - 1];
        } else {
            result += water;
        }
    }

    println!("{}", result);
}
