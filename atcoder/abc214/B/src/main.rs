use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
    }

    let mut result = 0;

    for a in 0..=100 {
        if a > s {
            break;
        }

        for b in 0..=100 {
            if a + b > s {
                break;
            }

            for c in 0..=100 {
                if a + b + c > s || a * b * c > t {
                    break;
                }

                result += 1;
            }
        }
    }

    println!("{}", result);
}
