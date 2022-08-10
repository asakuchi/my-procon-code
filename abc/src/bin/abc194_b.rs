use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut result = a_b[0].0 + a_b[0].1;

    for &(a, b) in &a_b {
        result = result.min(a + b);
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let a = a_b[i].0;
            let b = a_b[j].1;

            result = result.min(a.max(b));
        }
    }

    println!("{}", result);
}
