use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut result = 0;

    for x in 1..=1_000 {
        let mut ok = true;

        for (&a, &b) in a.iter().zip(b.iter()) {
            if a > x || x > b {
                ok = false;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
