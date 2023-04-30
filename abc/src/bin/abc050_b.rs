use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        p_x: [(Usize1, usize); m],
    }

    for &(p, x) in &p_x {
        let mut total = 0;

        for i in 0..n {
            if i == p {
                total += x;
            } else {
                total += t[i];
            }
        }

        println!("{}", total);
    }
}
