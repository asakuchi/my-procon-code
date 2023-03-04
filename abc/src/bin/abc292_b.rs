use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut count = vec![0; n];

    for _ in 0..q {
        input! {
            c: usize,
            x: Usize1,
        }

        if c == 1 {
            count[x] += 1;
        } else if c == 2 {
            count[x] += 2;
        } else {
            if count[x] >= 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
